use libloading::{Library, Symbol};

use crate::{types, Error, Result};

pub fn build_type_a_uid(uid: &[u8]) -> Result<types::NfaListenGfg> {
    let len = uid.len();

    if len != 0 || len != 4 || len != 7 || len != 10 {
        return Err(Error::ErrorUidLen);
    }

    let mut la_nfcid1 = [0u8; 10];

    la_nfcid1.copy_from_slice(uid);

    Ok(types::NfaListenGfg {
        la_enable: true,
        la_bit_frame_sdd: 0x04,
        la_platform_config: 0x0C,
        la_sel_info: 0,
        la_nfcid1_len: len as u8,
        la_nfcid1,
        ..Default::default()
    })
}

pub fn set_uid(path: &str, uid: &[u8]) -> Result<()> {
    let cfg = build_type_a_uid(uid)?;

    let library = unsafe { Library::new(path)? };

    let symbol: Symbol<NfaRequestExclusiveRfControl> = unsafe {
        library.get(b"_Z29NFA_RequestExclusiveRfControlhP15tNFA_LISTEN_CFGPFvhP18tNFA_CONN_EVT_DATAEPFvhP18tNFA_NDEF_EVT_DATAE\n")?
    };

    let code = unsafe {
        let cfg = (&cfg) as *const types::NfaListenGfg;

        symbol(
            types::NFA_TECHNOLOGY_MASK_A,
            cfg,
            types::common_callback,
            types::common_callback,
        )
    };

    if code == 0 {
        Ok(())
    } else {
        Err(Error::NfaStatus(code))
    }
}

pub fn reset_uid(path: &str) -> Result<()> {
    let library = unsafe { Library::new(path)? };

    let symbol: Symbol<NfaReleaseExclusiveRfControl> =
        unsafe { library.get(b"_Z29NFA_ReleaseExclusiveRfControlv\n")? };

    let code = unsafe { symbol() };

    if code == 0 {
        Ok(())
    } else {
        Err(Error::NfaStatus(code))
    }
}

type NfaRequestExclusiveRfControl = unsafe extern "C" fn(
    mask: u8,
    cfg: *const types::NfaListenGfg,
    conn_callback: types::NfaCallback,
    ndef_callback: types::NfaCallback,
) -> u8;

type NfaReleaseExclusiveRfControl = unsafe extern "C" fn() -> u8;
