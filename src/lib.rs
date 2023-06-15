pub use crate::rtcm_parser::Rtcm;

pub mod rtcm_parser {
    use deku::prelude::*;

    pub enum Rtcm {
        Rtcm1001(Rtcm1001),
        Rtcm1002(Rtcm1002),
        Rtcm1003(Rtcm1003),
        Rtcm1004(Rtcm1004),
        Rtcm1005(Rtcm1005),
        Rtcm1006(Rtcm1006),
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
    pub struct RtcmMSM7Satellite {
        #[deku(bits = "8")]
        pub rough_range: u8,

        #[deku(bits = "4")]
        pub extented_satallite_info: u8,

        #[deku(bits = "10")]
        pub rough_ranges_modulo: u16,

        #[deku(bits = "14")]
        pub rough_phase_range_rates: u16,
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
}
