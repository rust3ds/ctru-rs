//TODO: Implement C macro functions? Maybe?

//Result code level values
pub const RL_SUCCESS :i32 = 0;
pub const RL_INFO :i32 = 1;
pub const RL_FATAL :i32 = 31;
pub const RL_RESET :i32 = 30;
pub const RL_REINITIALIZE :i32 = 29;
pub const RL_USAGE :i32 = 28;
pub const RL_PERMANENT :i32 = 27;
pub const RL_TEMPORARY :i32 = 26;
pub const RL_STATUS :i32 = 25;

//Result code summary values
pub const RS_SUCCESS :i32 = 0;
pub const RS_NOP :i32 = 1;
pub const RS_WOULDBLOCK :i32 = 2;
pub const RS_OUTOFRESOURCE :i32 = 3;
pub const RS_NOTFOUND :i32 = 4;
pub const RS_INVALIDSTATE :i32 = 5;
pub const RS_NOTSUPPORTED :i32 = 6;
pub const RS_INVALIDARG :i32 = 7;
pub const RS_WRONGARG :i32 = 8;
pub const RS_CANCELED :i32 = 9;
pub const RS_STATUSCHANGED :i32 = 10;
pub const RS_INTERNAL :i32 = 11;
pub const RS_INVALIDRESVAL :i32 = 63;

//Result code generic description values
pub const RD_SUCCESS :i32 = 0;
pub const RD_INVALID_RESULT_VALUE :i32 = 1023;
pub const RD_TIMEOUT :i32 = 1022;
pub const RD_OUT_OF_RANGE :i32 = 1021;
pub const RD_ALREADY_EXISTS :i32 = 1020;
pub const RD_CANCEL_REQUESTED :i32 = 1019;
pub const RD_NOT_FOUND :i32 = 1018;
pub const RD_ALREADY_INITIALIZED :i32 = 1017;
pub const RD_NOT_INITIALIZED :i32 = 1016;
pub const RD_INVALID_HANDLE :i32 = 1015;
pub const RD_INVALID_POINTER :i32 = 1014;
pub const RD_INVALID_ADDRESS :i32 = 1013;
pub const RD_NOT_IMPLEMENTED :i32 = 1012;
pub const RD_OUT_OF_MEMORY :i32 = 1011;
pub const RD_MISALIGNED_SIZE :i32 = 1010;
pub const RD_MISALIGNED_ADDRESS :i32 = 1009;
pub const RD_BUSY :i32 = 1008;
pub const RD_NO_DATA :i32 = 1007;
pub const RD_INVALID_COMBINATION :i32 = 1006;
pub const RD_INVALID_ENUM_VALUE :i32 = 1005;
pub const RD_INVALID_SIZE :i32 = 1004;
pub const RD_ALREADY_DONE :i32 = 1003;
pub const RD_NOT_AUTHORIZED :i32 = 1002;
pub const RD_TOO_LARGE :i32 = 1001;
pub const RD_INVALID_SELECTION :i32 = 1000;
