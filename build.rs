fn main() {
    #[cfg(windows)]
    windows::build!(
        //windows::foundation::*,
        windows::devices::midi::*,
        windows::devices::enumeration::DeviceInformation,
        windows::storage::streams::{Buffer, DataWriter},
        windows::win32::multimedia::{midiInAddBuffer, midiInClose, midiInGetDevCapsW, midiInGetNumDevs,
            midiInOpen, midiInPrepareHeader, midiInReset, midiInStart,
            midiInStop, midiInUnprepareHeader, midiOutClose,
            midiOutGetDevCapsW, midiOutGetNumDevs, midiOutLongMsg, midiOutOpen,
            midiOutPrepareHeader, midiOutReset, midiOutShortMsg,
            midiOutUnprepareHeader, midiInMessage, midiOutMessage,
            HMIDIIN, HMIDIOUT, MIDIHDR, MIDIINCAPSW, MIDIOUTCAPSW,
            CALLBACK_FUNCTION, CALLBACK_NULL, MMSYSERR_NOERROR,
            MM_MIM_DATA, MM_MIM_LONGDATA, MM_MIM_LONGERROR,
            // more constants:
            // MIDIERR_NOTREADY, MIDIERR_STILLPLAYING, MMSYSERR_BADDEVICEID,
            // MMSYSERR_ALLOCATED, DRV_QUERYDEVICEINTERFACE, DRV_QUERYDEVICEINTERFACESIZE
    },
    );
  }