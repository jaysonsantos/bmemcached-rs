bitflags! {
        pub struct StoredType: u32 {
            const MTYPE_STRING          = 1 << 0;
            const MTYPE_U8              = 1 << 1;
            const MTYPE_U16             = 1 << 2;
            const MTYPE_U32             = 1 << 3;
            const MTYPE_U64             = 1 << 4;
            #[allow(dead_code)]
            const MTYPE_VECTOR          = 1 << 5;
            #[allow(dead_code)]
            const MTYPE_COMPRESSED      = 1 << 6;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_1  = 1 << 10;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_2  = 1 << 11;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_3  = 1 << 13;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_4  = 1 << 14;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_5  = 1 << 15;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_6  = 1 << 16;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_7  = 1 << 17;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_8  = 1 << 18;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_9  = 1 << 19;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_10 = 1 << 20;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_11 = 1 << 21;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_12 = 1 << 22;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_13 = 1 << 23;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_14 = 1 << 24;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_15 = 1 << 25;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_16 = 1 << 26;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_17 = 1 << 27;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_18 = 1 << 28;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_19 = 1 << 29;
            #[allow(dead_code)]
            const MTYPE_USER_DEFINED_20 = 1 << 30;
    }
}
