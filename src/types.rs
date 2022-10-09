pub type NfaTechnologyMask = u8;

#[repr(C)]
pub struct NfaListenGfg {
    pub la_enable: bool,
    pub la_bit_frame_sdd: u8,
    pub la_platform_config: u8,
    pub la_sel_info: u8,
    pub la_nfcid1_len: u8,
    pub la_nfcid1: [u8; 10],

    pub lb_enable: bool,
    pub lb_sensb_info: u8,
    pub lb_nfcid0_len: u8,
    pub lb_nfcid0: [u8; 4],
    pub lb_app_data: [u8; 4],
    pub lb_sfgi: u8,
    pub lb_adc_fo: u8,

    pub lf_enable: bool,
    pub lf_con_bitr_f: u8,
    pub lf_protocol_type: u8,
    pub lf_t3t_flags: u16,
    pub lf_t3t_identifier: [u8; 10],
    pub lf_t3t_pmm: [u8; 8],

    pub li_enable: bool,
    pub li_fwi: u8,
    pub la_hist_bytes_len: u8,
    pub la_hist_bytes: [u8; 15],
    pub lb_h_info_resp_len: u8,
    pub lb_h_info_resp: [u8; 15],

    pub ln_enable: bool,
    pub ln_wt: u8,
    pub ln_atr_res_gen_bytes_len: u8,
    pub ln_atr_res_gen_bytes: [u8; 48],
    pub ln_atr_res_config: u8,
}

extern "C" {
    pub fn NFA_RequestExclusiveRfControl(
        poll_mask: NfaTechnologyMask,
        p_listen_cfg: *const NfaListenGfg,
    ) -> u8;
}
