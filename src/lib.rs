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
    pub struct RtcmHeaderGPSRTK {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        reference_station_id: u16,

        #[deku(bits = "30")]
        gps_epoch_time: u32,

        #[deku(bits = "1")]
        synchronous_gnss_flag: u8,

        #[deku(bits = "5")]
        num_gps_satellite_signals_processed: u8,

        #[deku(bits = "1")]
        gps_divergence_free_smoothing_indicator: u8,

        #[deku(bits = "3")]
        gps_smoothing_interval: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1001Satellite {
        #[deku(bits = "6")]
        gps_satellite_id: u8,

        #[deku(bits = "1")]
        gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        gps_l1_lock_time_indicator: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1001 {
        pub header: RtcmHeaderGPSRTK,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        satellites: Vec<Rtcm1001Satellite>,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1002Satellite {
        #[deku(bits = "6")]
        gps_satellite_id: u8,

        #[deku(bits = "1")]
        gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        gps_l1_lock_time_indicator: u8,

        #[deku(bits = "8")]
        gps_integer_l1_pseudorange_modulus_ambiguity: u8,

        #[deku(bits = "8")]
        gps_l1_cnr: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1002 {
        pub header: RtcmHeaderGPSRTK,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        satellites: Vec<Rtcm1002Satellite>,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1003Satellite {
        #[deku(bits = "6")]
        gps_satellite_id: u8,

        #[deku(bits = "1")]
        gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        gps_l1_lock_time_indicator: u8,

        #[deku(bits = "2")]
        gps_l2_code_indicator: u8,

        #[deku(bits = "14")]
        gps_l2_l1_pseudorange_difference: i16,

        #[deku(bits = "20")]
        gps_l2_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        gps_l2_lock_time_indicator: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1003 {
        pub header: RtcmHeaderGPSRTK,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        satellites: Vec<Rtcm1003Satellite>,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1004Satellite {
        #[deku(bits = "6")]
        gps_satellite_id: u8,

        #[deku(bits = "1")]
        gps_l1_code_indicator: u8,

        #[deku(bits = "24")]
        gps_l1_pseudorange: u32,

        #[deku(bits = "20")]
        gps_l1_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        gps_l1_lock_time_indicator: u8,

        #[deku(bits = "8")]
        gps_integer_l1_pseudorange_modulus_ambiguity: u8,

        #[deku(bits = "8")]
        gps_l1_cnr: u8,

        #[deku(bits = "2")]
        gps_l2_code_indicator: u8,

        #[deku(bits = "14")]
        gps_l2_l1_pseudorange_difference: i16,

        #[deku(bits = "20")]
        gps_l2_phaserange_minus_pseudorange: i32,

        #[deku(bits = "7")]
        gps_l2_lock_time_indicator: u8,

        #[deku(bits = "8")]
        gps_l2_cnr: u8,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct Rtcm1004 {
        pub header: RtcmHeaderGPSRTK,

        #[deku(count = "header.num_gps_satellite_signals_processed")]
        satellites: Vec<Rtcm1004Satellite>,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1005 {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        reference_station_id: u16,

        #[deku(bits = "6")]
        reserved_realization_year: u8,

        #[deku(bits = "1")]
        gps_indicator: u8,

        #[deku(bits = "1")]
        glonass_indicator: u8,

        #[deku(bits = "1")]
        galileo_indicator: u8,

        #[deku(bits = "1")]
        reference_station_indicator: u8,

        #[deku(bits = "38")]
        antenna_reference_point_ecef_x: i64,

        #[deku(bits = "1")]
        single_receiver_oscillator_indicator: u8,

        #[deku(bits = "1")]
        reserved: u8,

        #[deku(bits = "38")]
        antenna_reference_point_ecef_y: i64,

        #[deku(bits = "2")]
        quarter_cycle_indicator: u8,

        #[deku(bits = "38")]
        antenna_reference_point_ecef_z: i64,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct Rtcm1006 {
        #[deku(bits = "12")]
        pub message_number: u16,

        #[deku(bits = "12")]
        reference_station_id: u16,

        #[deku(bits = "6")]
        realization_year: u8,

        #[deku(bits = "1")]
        gps_indicator: u8,

        #[deku(bits = "1")]
        glonass_indicator: u8,

        #[deku(bits = "1")]
        reserved_galileo_indicator: u8,

        #[deku(bits = "1")]
        reference_station_indicator: u8,

        #[deku(bits = "38")]
        antenna_reference_point_ecef_x: i64,

        #[deku(bits = "1")]
        single_receiver_oscillator_indicator: u8,

        #[deku(bits = "1")]
        reserved: u8,

        #[deku(bits = "38")]
        antenna_reference_point_ecef_y: i64,

        #[deku(bits = "2")]
        quarter_cycle_indicator: u8,

        #[deku(bits = "38")]
        antenna_reference_point_ecef_z: i64,

        #[deku(bits = "16")]
        antenna_height: u16,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct SatelliteData {
        #[deku(bits = "8")]
        rough_range: u8,
        #[deku(bits = "4")]
        ext_sat_info: u8,
        #[deku(bits = "10")]
        rough_range_milli: u16,
        #[deku(bits = "14")]
        rough_phase: u16,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct SignalData {
        #[deku(bits = "20")]
        pseudorange: i32,
        #[deku(bits = "24")]
        phaserange: i32,
        #[deku(bits = "10")]
        phaserange_lock: u16,
        #[deku(bits = "1")]
        halfcycle_ambiguity: bool,
        #[deku(bits = "10")]
        cnr: u16,
        #[deku(bits = "15")]
        phaserange_rate: i16,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    #[deku(endian = "big")]
    pub struct MsmHeader {
        #[deku(bits = "12")]
        pub msg_id: u16,
        #[deku(bits = "12")]
        station_id: u16,
        #[deku(bits = "30")]
        epoch: u32,
        #[deku(bits = "1")]
        multiple_msg: bool,
        #[deku(bits = "3")]
        iods: u8,
        #[deku(bits = "7")]
        reserved: u8,
        #[deku(bits = "2")]
        clock_steering: u8,
        #[deku(bits = "2")]
        external_clock_indicator: u8,
        #[deku(bits = "1")]
        smoothing_indicator: bool,
        #[deku(bits = "3")]
        smoothing_interval: u8,
        #[deku(bits = "64")]
        satellite_mask: u64,
        #[deku(bits = "32")]
        signal_mask: u32,
        #[deku(
            bits = 1,
            bits_read = "satellite_mask.count_ones() * signal_mask.count_ones()"
        )]
        cell_mask: Vec<bool>,
    }

    #[derive(Debug, DekuRead, DekuWrite)]
    pub struct RtcmMSM7 {
        pub header: MsmHeader,

        #[deku(count = "header.satellite_mask.count_ones()")]
        satellites: Vec<SatelliteData>,
        #[deku(count = "header.cell_mask.iter().filter(|&n| *n).count()")]
        signals: Vec<SignalData>,

        #[deku(bits = "1", count = "deku::rest.len()")]
        pub padding: Vec<bool>,
    }
}
