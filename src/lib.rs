pub mod rtcm_parser {
    use deku::prelude::*;

    use itertools::izip;
    use std::fmt;

    const RTCM_MAGIC: u8 = 0b11010011;
    const RTCM_PREAMBLE_SIZE: usize = 3;
    const RTCM_CHECKSUM_SIZE: usize = 3;

    // RtcmParser
    pub struct RtcmParser {
        buffer: Vec<u8>,
        max_size: usize,
        crc24: crc_any::CRC,
    }

    impl RtcmParser {
        pub fn new() -> Self {
            RtcmParser {
                buffer: vec![],
                max_size: 10000,
                crc24: crc_any::CRC::create_crc(0b1100001100100110011111011, 24, 0, 0, false),
            }
        }

        fn get_message_length(bytes: &[u8]) -> usize {
            let rtcm_length = u16::from_be_bytes([bytes[0], bytes[1]]) & 0x3FFu16;
            rtcm_length as usize
        }

        pub fn parse(&mut self, input: &[u8]) -> Vec<Vec<u8>> {
            // Update buffer
            self.buffer.extend_from_slice(input);

            // Initialize result
            let mut result: Vec<Vec<u8>> = Vec::new();

            // Scan for RTCM preamble
            let mut draining_point = 0;
            for i in 0..=self.buffer.len() - RTCM_PREAMBLE_SIZE - RTCM_CHECKSUM_SIZE {
                if self.buffer[i] != RTCM_MAGIC {
                    continue;
                }

                let rtcm_length = Self::get_message_length(&self.buffer[i + 1..i + 3]);

                // Read message length (stored in 10 bits)
                if i + 6 + rtcm_length >= self.buffer.len() {
                    continue; // This might not be a real message so the rest of the buffer still need to be checked
                }

                let message_candidate =
                    &self.buffer[i..i + RTCM_PREAMBLE_SIZE + rtcm_length + RTCM_CHECKSUM_SIZE];

                // Compute the checksum using crc24q
                self.crc24.reset();
                self.crc24
                    .digest(&message_candidate[..RTCM_PREAMBLE_SIZE + rtcm_length]);
                let checksum_computed = self.crc24.get_crc();
                // Extract the checksum from the message
                let checksum_message =
                    ((message_candidate[RTCM_PREAMBLE_SIZE + rtcm_length] as u64) << 16)
                        | ((message_candidate[RTCM_PREAMBLE_SIZE + rtcm_length + 1] as u64) << 8)
                        | (message_candidate[RTCM_PREAMBLE_SIZE + rtcm_length + 2] as u64);

                if checksum_computed != checksum_message {
                    continue; // Bad checksum
                }

                result.push(
                    self.buffer[i..i + RTCM_PREAMBLE_SIZE + rtcm_length + RTCM_CHECKSUM_SIZE]
                        .to_vec(),
                );
                draining_point = i + RTCM_PREAMBLE_SIZE + rtcm_length + RTCM_CHECKSUM_SIZE;
            }

            // Update the draining point to satisfy the max_size
            if self.buffer.len() - draining_point > self.max_size {
                draining_point = self.buffer.len() - self.max_size;
            }

            self.buffer.drain(..draining_point);

            result
        }
    }

    // RTCM Data structures
    #[derive(Debug)]
    pub enum Rtcm {
        Rtcm1001(Rtcm1001),
        Rtcm1002(Rtcm1002),
        Rtcm1003(Rtcm1003),
        Rtcm1004(Rtcm1004),
        Rtcm1005(Rtcm1005),
        Rtcm1006(Rtcm1006),
        Rtcm1019(Rtcm1019),
        RtcmMSM7(RtcmMSM7),
        UnsupportedType(u16),
    }

    impl Rtcm {
        pub fn parse(data: &[u8]) -> Result<Rtcm, deku::error::DekuError> {
            // First 12 bits contain the message type
            let msg_id = u16::from(data[0]) << 4 | (u16::from(data[1]) >> 4);
            match msg_id {
                1001 => Rtcm1001::try_from(data).map(|rtcm| Rtcm::Rtcm1001(rtcm)),
                1002 => Rtcm1002::try_from(data).map(|rtcm| Rtcm::Rtcm1002(rtcm)),
                1003 => Rtcm1003::try_from(data).map(|rtcm| Rtcm::Rtcm1003(rtcm)),
                1004 => Rtcm1004::try_from(data).map(|rtcm| Rtcm::Rtcm1004(rtcm)),
                1005 => Rtcm1005::try_from(data).map(|rtcm| Rtcm::Rtcm1005(rtcm)),
                1006 => Rtcm1006::try_from(data).map(|rtcm| Rtcm::Rtcm1006(rtcm)),
                1019 => Rtcm1019::try_from(data).map(|rtcm| Rtcm::Rtcm1019(rtcm)),
                1077 | 1087 | 1097 => RtcmMSM7::try_from(data).map(|rtcm| Rtcm::RtcmMSM7(rtcm)),
                any => Ok(Rtcm::UnsupportedType(any)),
            }
        }
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct RtcmHeader {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        pub reference_station_id: u16,

        #[deku(bits = "30")]
        pub gps_epoch_time: u32,

        #[deku(bits = "1")]
        pub synchronous_gnss_flag: u8,

        #[deku(bits = "5")]
        pub num_gps_satellite_signals_processed: u8,

        #[deku(bits = "1")]
        pub gps_divergence_free_smoothing_indicator: u8,

        #[deku(bits = "3")]
        pub gps_smoothing_interval: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1001Satellite {
        #[deku(bits = "6")]
        pub gps_satellite_id: u8,

        #[deku(bits = "1")]
        pub gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        pub gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        pub gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        pub gps_l1_lock_time_indicator: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1001 {
        pub header: RtcmHeader,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        pub satellites: Vec<Rtcm1001Satellite>,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1002Satellite {
        #[deku(bits = "6")]
        pub gps_satellite_id: u8,

        #[deku(bits = "1")]
        pub gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        pub gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        pub gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        pub gps_l1_lock_time_indicator: u8,

        #[deku(bits = "8")]
        pub gps_integer_l1_pseudorange_modulus_ambiguity: u8,

        #[deku(bits = "8")]
        pub gps_l1_cnr: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1002 {
        pub header: RtcmHeader,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        pub satellites: Vec<Rtcm1002Satellite>,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1003Satellite {
        #[deku(bits = "6")]
        pub gps_satellite_id: u8,

        #[deku(bits = "1")]
        pub gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        pub gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        pub gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        pub gps_l1_lock_time_indicator: u8,

        #[deku(bits = "2")]
        pub gps_l2_code_indicator: u8,

        #[deku(bits = "14")]
        pub gps_l2_l1_pseudorange_difference: i16,

        #[deku(bits = "20")]
        pub gps_l2_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        pub gps_l2_lock_time_indicator: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1003 {
        pub header: RtcmHeader,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        pub satellites: Vec<Rtcm1003Satellite>,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1004Satellite {
        #[deku(bits = "6")]
        pub gps_satellite_id: u8,

        #[deku(bits = "1")]
        pub gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        pub gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        pub gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        pub gps_l1_lock_time_indicator: u8,

        #[deku(bits = "8")]
        pub gps_integer_l1_pseudorange_modulus_ambiguity: u8,

        #[deku(bits = "8")]
        pub gps_l1_cnr: u8,

        #[deku(bits = "2")]
        pub gps_l2_code_indicator: u8,

        #[deku(bits = "14")]
        pub gps_l2_l1_pseudorange_difference: i16,

        #[deku(bits = "20")]
        pub gps_l2_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        pub gps_l2_lock_time_indicator: u8,

        #[deku(bits = "8")]
        pub gps_l2_cnr: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1004 {
        pub header: RtcmHeader,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        pub satellites: Vec<Rtcm1004Satellite>,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1005 {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        pub reference_station_id: u16,

        #[deku(bits = "6")]
        pub itrf_realization_year: u8,

        #[deku(bits = "1")]
        pub gps_indicator: u8,

        #[deku(bits = "1")]
        pub glonass_indicator: u8,

        #[deku(bits = "1")]
        pub galileo_indicator: u8,

        #[deku(bits = "1")]
        pub reference_station_indicator: u8,

        #[deku(bits = "38")]
        pub antenna_reference_point_ecef_x: i64,

        #[deku(bits = "1")]
        pub single_receiver_oscillator_indicator: u8,

        #[deku(bits = "1")]
        pub reserved: u8,

        #[deku(bits = "38")]
        pub antenna_reference_point_ecef_y: i64,

        #[deku(bits = "2")]
        pub quarter_cycle_indicator: u8,

        #[deku(bits = "38")]
        pub antenna_reference_point_ecef_z: i64,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1006 {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        pub reference_station_id: u16,

        #[deku(bits = "6")]
        pub realization_year: u8,

        #[deku(bits = "1")]
        pub gps_indicator: u8,

        #[deku(bits = "1")]
        pub glonass_indicator: u8,

        #[deku(bits = "1")]
        pub galileo_indicator: u8,

        #[deku(bits = "1")]
        pub reference_station_indicator: u8,

        #[deku(bits = "38")]
        pub antenna_reference_point_ecef_x: i64,

        #[deku(bits = "1")]
        pub single_receiver_oscillator_indicator: u8,

        #[deku(bits = "1")]
        pub reserved: u8,

        #[deku(bits = "38")]
        pub antenna_reference_point_ecef_y: i64,

        #[deku(bits = "2")]
        pub quarter_cycle_indicator: u8,

        #[deku(bits = "38")]
        pub antenna_reference_point_ecef_z: i64,

        #[deku(bits = "16")]
        pub antenna_height: u16,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1019 {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "6")]
        pub satellite_id: u8,

        #[deku(bits = "10")]
        pub week_number: u16,

        #[deku(bits = "4")]
        pub sv_accuracy: u8,

        #[deku(bits = "2")]
        pub code_on_l2: u8,

        #[deku(bits = "14")]
        pub idot: i16,

        #[deku(bits = "8")]
        pub iode: u8,

        #[deku(bits = "16")]
        pub t_oc: u16,

        #[deku(bits = "8")]
        pub a_f2: i8,

        #[deku(bits = "16")]
        pub a_f1: i16,

        #[deku(bits = "22")]
        pub a_f0: i32,

        #[deku(bits = "10")]
        pub iocd: u16,

        #[deku(bits = "16")]
        pub c_rs: i16,

        #[deku(bits = "16")]
        pub delta_n: i16,

        #[deku(bits = "32")]
        pub m0: i32,

        #[deku(bits = "16")]
        pub c_uc: i16,

        #[deku(bits = "32")]
        pub eccentricity: u32,

        #[deku(bits = "16")]
        pub c_us: i16,

        #[deku(bits = "32")]
        pub a_sqrt: u32,

        #[deku(bits = "16")]
        pub t_oe: u16,

        #[deku(bits = "16")]
        pub c_ic: i16,

        #[deku(bits = "32")]
        pub omega0: i32,

        #[deku(bits = "16")]
        pub c_is: i16,

        #[deku(bits = "32")]
        pub i0: i32,

        #[deku(bits = "16")]
        pub c_rc: i16,

        #[deku(bits = "32")]
        pub omega: i32, // Argument of Perigee

        #[deku(bits = "24")]
        pub odmegadot: i32, // Rate of acension

        #[deku(bits = "8")]
        pub t_gd: i8,

        #[deku(bits = "6")]
        pub sv_health: u8,

        #[deku(bits = "1")]
        pub l2_p_data_flag: bool,

        #[deku(bits = "1")]
        pub fit_interval: bool,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct RtcmMSM7Satellite {
        #[deku(bits = "8")]
        pub rough_range: u8,

        #[deku(bits = "4")]
        pub extented_satallite_info: u8,

        #[deku(bits = "10")]
        pub rough_ranges_modulo: u16,

        #[deku(bits = "14")]
        pub rough_phase_range_rates: i16,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct RtcmMSM7Signal {
        #[deku(bits = "20")]
        pub fine_pseudorange: i32,

        #[deku(bits = "24")]
        pub fine_phase_range: i32,

        #[deku(bits = "10")]
        pub phaserange_lock_indicator: u16,

        #[deku(bits = "1")]
        pub halfcycle_ambiguity_indicator: bool,

        #[deku(bits = "10")]
        pub cnr: u16,

        #[deku(bits = "15")]
        pub fine_phase_range_rate: i16,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct MsmHeader {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        pub reference_station_id: u16,

        #[deku(bits = "30")]
        pub gnss_epoch_time: u32,

        #[deku(bits = "1")]
        pub multiple_message_bit: u8,

        #[deku(bits = "3")]
        pub iods_issue_of_data_station: u8,

        #[deku(bits = "7")]
        pub reserved: u8,

        #[deku(bits = "2")]
        pub clock_steering_indicator: u8,

        #[deku(bits = "2")]
        pub external_clock_indicator: u8,

        #[deku(bits = "1")]
        pub gnss_divergence_free_smoothing_indicator: u8,

        #[deku(bits = "3")]
        pub gnss_smoothing_interval: u8,

        #[deku(bits = "64")]
        pub gnss_satellite_mask: u64,

        #[deku(bits = "32")]
        pub gnss_signal_mask: u32,

        #[deku(
            bits = 1,
            bits_read = "gnss_satellite_mask.count_ones() * gnss_signal_mask.count_ones()"
        )]
        pub cell_mask: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct RtcmMSM7 {
        pub header: MsmHeader,

        #[deku(count = "header.gnss_satellite_mask.count_ones()")]
        pub satellites: Vec<RtcmMSM7Satellite>,

        #[deku(count = "header.cell_mask.iter().filter(|&n| *n).count()")]
        pub signals: Vec<RtcmMSM7Signal>,

        #[deku(bits = "1", count = "deku::rest.len() % 8")]
        pub padding: Vec<bool>,
    }

    fn pseudorange(sat: &RtcmMSM7Satellite, sig: &RtcmMSM7Signal) -> f64 {
        let c = 299_792_458.0;
        c / 1000.0
            * (sat.rough_ranges_modulo as f64
                + sat.rough_range as f64 / 1024.0
                + 2e-29 * sig.fine_pseudorange as f64)
    }

    fn phaserange(sat: &RtcmMSM7Satellite, sig: &RtcmMSM7Signal) -> f64 {
        let c = 299_792_458.0;
        c / 1000.0
            * (sat.rough_ranges_modulo as f64
                + sat.rough_range as f64 / 1024.0
                + 2e-31 * sig.fine_phase_range as f64)
    }

    fn phaserangerate(sat: &RtcmMSM7Satellite, sig: &RtcmMSM7Signal) -> f64 {
        sat.rough_phase_range_rates as f64 + 0.0001 * sig.fine_phase_range_rate as f64
    }

    impl fmt::Display for RtcmMSM7 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Get number of signals
            let sat_id: Vec<usize> = (1..=64)
                .filter(|id| ((1u64 << (64 - id)) & self.header.gnss_satellite_mask) != 0)
                .collect();

            let n_sig = self.header.gnss_signal_mask.count_ones() as usize;
            let n_sig_per_sat: Vec<usize> = self
                .header
                .cell_mask
                .chunks(n_sig)
                .map(|sig_for_sat| sig_for_sat.iter().filter(|&mask| *mask).count())
                .collect();

            // Create iteror (repeate satellite for each signal)
            let iter_sat: Vec<&RtcmMSM7Satellite> = std::iter::zip(&self.satellites, n_sig_per_sat)
                .flat_map(|(sat, count)| std::iter::repeat(sat).take(count))
                .collect();

            for (id, sat, sig) in izip!(1..64, iter_sat, self.signals.iter()) {
                let range = pseudorange(sat, sig);
                let phase_range = phaserange(sat, sig);
                let phase_range_rate = phaserangerate(sat, sig);
                let cnr: f64 = (2.0 as f64).powi(-4) * sig.cnr as f64;
                writeln!(f, "{id} Pseudorange: {range}, PhaseRange: {phase_range}, PhaseRangeRate: {phase_range_rate}, CNR: {cnr}dBHz")?;
            }

            Ok(())
        }
    }
}
