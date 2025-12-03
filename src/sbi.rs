use core::arch::asm;

/**tipo di Err in SbiResult di qualsiasi SBI call andata male.
Pui confrotare la SbiError con i valori, ma meglio usare i valore della enum.
*/
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbiError{
    SUCCES         = 0 ,//una SBI call da errore solo se il valore e' < 0
                        //quinidi includo lo zero solo per far capire che zero in realta non e un errore
    Failed         = -1,
    NotSupported   = -2,
    InvalidParam   = -3,
    Denied         = -4,
    InvalidAddres  = -5,
    AlreadyAvaible = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
    NoSHMEM        = -9,
    InvalidState   = -10,
    BadRange       = -11,
    Timeout        = -12,
    IO             = -13,
    DeniedLocked   = -14,
}

impl SbiError{
    fn new(err: isize) -> Self{
        match err {
            0..=isize::MAX => Self::SUCCES, //non dovrebbe mai accadere ma non e' in se e per se un errore
            -1 => Self::Failed,
            -2 => Self::NotSupported,
            -3 => Self::InvalidParam,
            -4 => Self::Denied,
            -5 => Self::InvalidAddres,
            -6 => Self::AlreadyAvaible,
            -7 => Self::AlreadyStarted,
            -8 => Self::AlreadyStopped,
            -9 => Self::NoSHMEM,
            -10 => Self::InvalidState,
            -11 => Self::BadRange,
            -12 => Self::Timeout,
            -13 => Self::IO,
            -14 => Self::DeniedLocked,
            _ =>  panic!("Trovato un errore che non e' compreso tra 0 e -14"),
        }
    }
}

///tipo di Ok(T) di SbiResult
pub struct SbiVal(usize);


impl SbiVal{
    fn new(val: usize) -> Self {
        Self(val)
    }

    pub fn as_usize(self) -> usize {
        self.0
    }

    pub fn as_isize(self) -> isize {
        usize::cast_signed(self.0)
    }
}


///tipo di ritorno da qualsiasi SBI call
pub type SbiResult = core::result::Result<SbiVal, SbiError>;


#[inline(always)]
pub fn sbi_call(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize,
                fid: usize, eid: usize) -> SbiResult{
    let mut err: isize;
    let mut ret: usize;
    unsafe{
        asm!{
            "ecall",
            inout("a0") arg0 => err,
            inout("a1") arg1 => ret,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
        }
    }

    if err < 0 {
        return Err(SbiError::new(err));
    }
    else {
        return Ok(SbiVal::new(ret));
    }
}

pub fn putc(c:u8) -> (){
    sbi_call(c as usize, 0, 0, 0, 0, 0, 0, 0x01);
}
