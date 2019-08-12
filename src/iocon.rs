#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for port 0 pins PIO0_0"]
    pub pio00: PIO00,
    #[doc = "0x04 - Digital I/O control for port 0 pins PIO0_1"]
    pub pio01: PIO01,
    #[doc = "0x08 - Digital I/O control for port 0 pins PIO0_2"]
    pub pio02: PIO02,
    #[doc = "0x0c - Digital I/O control for port 0 pins PIO0_3"]
    pub pio03: PIO03,
    #[doc = "0x10 - Digital I/O control for port 0 pins PIO0_4"]
    pub pio04: PIO04,
    #[doc = "0x14 - Digital I/O control for port 0 pins PIO0_5"]
    pub pio05: PIO05,
    #[doc = "0x18 - Digital I/O control for port 0 pins PIO0_6"]
    pub pio06: PIO06,
    #[doc = "0x1c - Digital I/O control for port 0 pins PIO0_7"]
    pub pio07: PIO07,
    #[doc = "0x20 - Digital I/O control for port 0 pins PIO0_8"]
    pub pio08: PIO08,
    #[doc = "0x24 - Digital I/O control for port 0 pins PIO0_9"]
    pub pio09: PIO09,
    #[doc = "0x28 - Digital I/O control for port 0 pins PIO0_10"]
    pub pio010: PIO010,
    #[doc = "0x2c - Digital I/O control for port 0 pins PIO0_11"]
    pub pio011: PIO011,
    #[doc = "0x30 - Digital I/O control for port 0 pins PIO0_12"]
    pub pio012: PIO012,
    #[doc = "0x34 - Digital I/O control for port 0 pins PIO0_13"]
    pub pio013: PIO013,
    #[doc = "0x38 - Digital I/O control for port 0 pins PIO0_14"]
    pub pio014: PIO014,
    #[doc = "0x3c - Digital I/O control for port 0 pins PIO0_15"]
    pub pio015: PIO015,
    #[doc = "0x40 - Digital I/O control for port 0 pins PIO0_16"]
    pub pio016: PIO016,
    #[doc = "0x44 - Digital I/O control for port 0 pins PIO0_17"]
    pub pio017: PIO017,
    #[doc = "0x48 - Digital I/O control for port 0 pins PIO0_18"]
    pub pio018: PIO018,
    #[doc = "0x4c - Digital I/O control for port 0 pins PIO0_19"]
    pub pio019: PIO019,
    #[doc = "0x50 - Digital I/O control for port 0 pins PIO0_20"]
    pub pio020: PIO020,
    #[doc = "0x54 - Digital I/O control for port 0 pins PIO0_21"]
    pub pio021: PIO021,
    #[doc = "0x58 - Digital I/O control for port 0 pins PIO0_22"]
    pub pio022: PIO022,
    #[doc = "0x5c - Digital I/O control for port 0 pins PIO0_23"]
    pub pio023: PIO023,
    #[doc = "0x60 - Digital I/O control for port 0 pins PIO0_24"]
    pub pio024: PIO024,
    #[doc = "0x64 - Digital I/O control for port 0 pins PIO0_25"]
    pub pio025: PIO025,
    #[doc = "0x68 - Digital I/O control for port 0 pins PIO0_26"]
    pub pio026: PIO026,
    #[doc = "0x6c - Digital I/O control for port 0 pins PIO0_27"]
    pub pio027: PIO027,
    #[doc = "0x70 - Digital I/O control for port 0 pins PIO0_28"]
    pub pio028: PIO028,
    #[doc = "0x74 - Digital I/O control for port 0 pins PIO0_29"]
    pub pio029: PIO029,
    #[doc = "0x78 - Digital I/O control for port 0 pins PIO0_30"]
    pub pio030: PIO030,
    #[doc = "0x7c - Digital I/O control for port 0 pins PIO0_31"]
    pub pio031: PIO031,
    #[doc = "0x80 - Digital I/O control for port 1 pins PIO1_0"]
    pub pio10: PIO10,
    #[doc = "0x84 - Digital I/O control for port 1 pins PIO1_1"]
    pub pio11: PIO11,
    #[doc = "0x88 - Digital I/O control for port 1 pins PIO1_2"]
    pub pio12: PIO12,
    #[doc = "0x8c - Digital I/O control for port 1 pins PIO1_3"]
    pub pio13: PIO13,
    #[doc = "0x90 - Digital I/O control for port 1 pins PIO1_4"]
    pub pio14: PIO14,
    #[doc = "0x94 - Digital I/O control for port 1 pins PIO1_5"]
    pub pio15: PIO15,
    #[doc = "0x98 - Digital I/O control for port 1 pins PIO1_6"]
    pub pio16: PIO16,
    #[doc = "0x9c - Digital I/O control for port 1 pins PIO1_7"]
    pub pio17: PIO17,
    #[doc = "0xa0 - Digital I/O control for port 1 pins PIO1_8"]
    pub pio18: PIO18,
    #[doc = "0xa4 - Digital I/O control for port 1 pins PIO1_9"]
    pub pio19: PIO19,
    #[doc = "0xa8 - Digital I/O control for port 1 pins PIO1_10"]
    pub pio110: PIO110,
    #[doc = "0xac - Digital I/O control for port 1 pins PIO1_11"]
    pub pio111: PIO111,
    #[doc = "0xb0 - Digital I/O control for port 1 pins PIO1_12"]
    pub pio112: PIO112,
    #[doc = "0xb4 - Digital I/O control for port 1 pins PIO1_13"]
    pub pio113: PIO113,
    #[doc = "0xb8 - Digital I/O control for port 1 pins PIO1_14"]
    pub pio114: PIO114,
    #[doc = "0xbc - Digital I/O control for port 1 pins PIO1_15"]
    pub pio115: PIO115,
    #[doc = "0xc0 - Digital I/O control for port 1 pins PIO1_16"]
    pub pio116: PIO116,
    #[doc = "0xc4 - Digital I/O control for port 1 pins PIO1_17"]
    pub pio117: PIO117,
    #[doc = "0xc8 - Digital I/O control for port 1 pins PIO1_18"]
    pub pio118: PIO118,
    #[doc = "0xcc - Digital I/O control for port 1 pins PIO1_19"]
    pub pio119: PIO119,
    #[doc = "0xd0 - Digital I/O control for port 1 pins PIO1_20"]
    pub pio120: PIO120,
    #[doc = "0xd4 - Digital I/O control for port 1 pins PIO1_21"]
    pub pio121: PIO121,
    #[doc = "0xd8 - Digital I/O control for port 1 pins PIO1_22"]
    pub pio122: PIO122,
    #[doc = "0xdc - Digital I/O control for port 1 pins PIO1_23"]
    pub pio123: PIO123,
    #[doc = "0xe0 - Digital I/O control for port 1 pins PIO1_24"]
    pub pio124: PIO124,
    #[doc = "0xe4 - Digital I/O control for port 1 pins PIO1_25"]
    pub pio125: PIO125,
    #[doc = "0xe8 - Digital I/O control for port 1 pins PIO1_26"]
    pub pio126: PIO126,
    #[doc = "0xec - Digital I/O control for port 1 pins PIO1_27"]
    pub pio127: PIO127,
    #[doc = "0xf0 - Digital I/O control for port 1 pins PIO1_28"]
    pub pio128: PIO128,
    #[doc = "0xf4 - Digital I/O control for port 1 pins PIO1_29"]
    pub pio129: PIO129,
    #[doc = "0xf8 - Digital I/O control for port 1 pins PIO1_30"]
    pub pio130: PIO130,
    #[doc = "0xfc - Digital I/O control for port 1 pins PIO1_31"]
    pub pio131: PIO131,
    #[doc = "0x100 - Digital I/O control for port 2 pins PIO2_0"]
    pub pio20: PIO20,
    #[doc = "0x104 - Digital I/O control for port 2 pins PIO2_1"]
    pub pio21: PIO21,
    #[doc = "0x108 - Digital I/O control for port 2 pins PIO2_2"]
    pub pio22: PIO22,
    #[doc = "0x10c - Digital I/O control for port 2 pins PIO2_3"]
    pub pio23: PIO23,
    #[doc = "0x110 - Digital I/O control for port 2 pins PIO2_4"]
    pub pio24: PIO24,
    #[doc = "0x114 - Digital I/O control for port 2 pins PIO2_5"]
    pub pio25: PIO25,
    #[doc = "0x118 - Digital I/O control for port 2 pins PIO2_6"]
    pub pio26: PIO26,
    #[doc = "0x11c - Digital I/O control for port 2 pins PIO2_7"]
    pub pio27: PIO27,
    #[doc = "0x120 - Digital I/O control for port 2 pins PIO2_8"]
    pub pio28: PIO28,
    #[doc = "0x124 - Digital I/O control for port 2 pins PIO2_9"]
    pub pio29: PIO29,
    #[doc = "0x128 - Digital I/O control for port 2 pins PIO2_10"]
    pub pio210: PIO210,
    #[doc = "0x12c - Digital I/O control for port 2 pins PIO2_11"]
    pub pio211: PIO211,
    #[doc = "0x130 - Digital I/O control for port 2 pins PIO2_12"]
    pub pio212: PIO212,
    #[doc = "0x134 - Digital I/O control for port 2 pins PIO2_13"]
    pub pio213: PIO213,
    #[doc = "0x138 - Digital I/O control for port 2 pins PIO2_14"]
    pub pio214: PIO214,
    #[doc = "0x13c - Digital I/O control for port 2 pins PIO2_15"]
    pub pio215: PIO215,
    #[doc = "0x140 - Digital I/O control for port 2 pins PIO2_16"]
    pub pio216: PIO216,
    #[doc = "0x144 - Digital I/O control for port 2 pins PIO2_17"]
    pub pio217: PIO217,
    #[doc = "0x148 - Digital I/O control for port 2 pins PIO2_18"]
    pub pio218: PIO218,
    #[doc = "0x14c - Digital I/O control for port 2 pins PIO2_19"]
    pub pio219: PIO219,
    #[doc = "0x150 - Digital I/O control for port 2 pins PIO2_20"]
    pub pio220: PIO220,
    #[doc = "0x154 - Digital I/O control for port 2 pins PIO2_21"]
    pub pio221: PIO221,
    #[doc = "0x158 - Digital I/O control for port 2 pins PIO2_22"]
    pub pio222: PIO222,
    #[doc = "0x15c - Digital I/O control for port 2 pins PIO2_23"]
    pub pio223: PIO223,
    #[doc = "0x160 - Digital I/O control for port 2 pins PIO2_24"]
    pub pio224: PIO224,
    #[doc = "0x164 - Digital I/O control for port 2 pins PIO2_25"]
    pub pio225: PIO225,
    #[doc = "0x168 - Digital I/O control for port 2 pins PIO2_26"]
    pub pio226: PIO226,
    #[doc = "0x16c - Digital I/O control for port 2 pins PIO2_27"]
    pub pio227: PIO227,
    #[doc = "0x170 - Digital I/O control for port 2 pins PIO2_28"]
    pub pio228: PIO228,
    #[doc = "0x174 - Digital I/O control for port 2 pins PIO2_29"]
    pub pio229: PIO229,
    #[doc = "0x178 - Digital I/O control for port 2 pins PIO2_30"]
    pub pio230: PIO230,
    #[doc = "0x17c - Digital I/O control for port 2 pins PIO2_31"]
    pub pio231: PIO231,
    #[doc = "0x180 - Digital I/O control for port 3 pins PIO3_0"]
    pub pio30: PIO30,
    #[doc = "0x184 - Digital I/O control for port 3 pins PIO3_1"]
    pub pio31: PIO31,
    #[doc = "0x188 - Digital I/O control for port 3 pins PIO3_2"]
    pub pio32: PIO32,
    #[doc = "0x18c - Digital I/O control for port 3 pins PIO3_3"]
    pub pio33: PIO33,
    #[doc = "0x190 - Digital I/O control for port 3 pins PIO3_4"]
    pub pio34: PIO34,
    #[doc = "0x194 - Digital I/O control for port 3 pins PIO3_5"]
    pub pio35: PIO35,
    #[doc = "0x198 - Digital I/O control for port 3 pins PIO3_6"]
    pub pio36: PIO36,
    #[doc = "0x19c - Digital I/O control for port 3 pins PIO3_7"]
    pub pio37: PIO37,
    #[doc = "0x1a0 - Digital I/O control for port 3 pins PIO3_8"]
    pub pio38: PIO38,
    #[doc = "0x1a4 - Digital I/O control for port 3 pins PIO3_9"]
    pub pio39: PIO39,
    #[doc = "0x1a8 - Digital I/O control for port 3 pins PIO3_10"]
    pub pio310: PIO310,
    #[doc = "0x1ac - Digital I/O control for port 3 pins PIO3_11"]
    pub pio311: PIO311,
    #[doc = "0x1b0 - Digital I/O control for port 3 pins PIO3_12"]
    pub pio312: PIO312,
    #[doc = "0x1b4 - Digital I/O control for port 3 pins PIO3_13"]
    pub pio313: PIO313,
    #[doc = "0x1b8 - Digital I/O control for port 3 pins PIO3_14"]
    pub pio314: PIO314,
    #[doc = "0x1bc - Digital I/O control for port 3 pins PIO3_15"]
    pub pio315: PIO315,
    #[doc = "0x1c0 - Digital I/O control for port 3 pins PIO3_16"]
    pub pio316: PIO316,
    #[doc = "0x1c4 - Digital I/O control for port 3 pins PIO3_17"]
    pub pio317: PIO317,
    #[doc = "0x1c8 - Digital I/O control for port 3 pins PIO3_18"]
    pub pio318: PIO318,
    #[doc = "0x1cc - Digital I/O control for port 3 pins PIO3_19"]
    pub pio319: PIO319,
    #[doc = "0x1d0 - Digital I/O control for port 3 pins PIO3_20"]
    pub pio320: PIO320,
    #[doc = "0x1d4 - Digital I/O control for port 3 pins PIO3_21"]
    pub pio321: PIO321,
    #[doc = "0x1d8 - Digital I/O control for port 3 pins PIO3_22"]
    pub pio322: PIO322,
    #[doc = "0x1dc - Digital I/O control for port 3 pins PIO3_23"]
    pub pio323: PIO323,
    #[doc = "0x1e0 - Digital I/O control for port 3 pins PIO3_24"]
    pub pio324: PIO324,
    #[doc = "0x1e4 - Digital I/O control for port 3 pins PIO3_25"]
    pub pio325: PIO325,
    #[doc = "0x1e8 - Digital I/O control for port 3 pins PIO3_26"]
    pub pio326: PIO326,
    #[doc = "0x1ec - Digital I/O control for port 3 pins PIO3_27"]
    pub pio327: PIO327,
    #[doc = "0x1f0 - Digital I/O control for port 3 pins PIO3_28"]
    pub pio328: PIO328,
    #[doc = "0x1f4 - Digital I/O control for port 3 pins PIO3_29"]
    pub pio329: PIO329,
    #[doc = "0x1f8 - Digital I/O control for port 3 pins PIO3_30"]
    pub pio330: PIO330,
    #[doc = "0x1fc - Digital I/O control for port 3 pins PIO3_31"]
    pub pio331: PIO331,
    #[doc = "0x200 - Digital I/O control for port 4 pins PIO4_0"]
    pub pio40: PIO40,
    #[doc = "0x204 - Digital I/O control for port 4 pins PIO4_1"]
    pub pio41: PIO41,
    #[doc = "0x208 - Digital I/O control for port 4 pins PIO4_2"]
    pub pio42: PIO42,
    #[doc = "0x20c - Digital I/O control for port 4 pins PIO4_3"]
    pub pio43: PIO43,
    #[doc = "0x210 - Digital I/O control for port 4 pins PIO4_4"]
    pub pio44: PIO44,
    #[doc = "0x214 - Digital I/O control for port 4 pins PIO4_5"]
    pub pio45: PIO45,
    #[doc = "0x218 - Digital I/O control for port 4 pins PIO4_6"]
    pub pio46: PIO46,
    #[doc = "0x21c - Digital I/O control for port 4 pins PIO4_7"]
    pub pio47: PIO47,
    #[doc = "0x220 - Digital I/O control for port 4 pins PIO4_8"]
    pub pio48: PIO48,
    #[doc = "0x224 - Digital I/O control for port 4 pins PIO4_9"]
    pub pio49: PIO49,
    #[doc = "0x228 - Digital I/O control for port 4 pins PIO4_10"]
    pub pio410: PIO410,
    #[doc = "0x22c - Digital I/O control for port 4 pins PIO4_11"]
    pub pio411: PIO411,
    #[doc = "0x230 - Digital I/O control for port 4 pins PIO4_12"]
    pub pio412: PIO412,
    #[doc = "0x234 - Digital I/O control for port 4 pins PIO4_13"]
    pub pio413: PIO413,
    #[doc = "0x238 - Digital I/O control for port 4 pins PIO4_14"]
    pub pio414: PIO414,
    #[doc = "0x23c - Digital I/O control for port 4 pins PIO4_15"]
    pub pio415: PIO415,
    #[doc = "0x240 - Digital I/O control for port 4 pins PIO4_16"]
    pub pio416: PIO416,
    #[doc = "0x244 - Digital I/O control for port 4 pins PIO4_17"]
    pub pio417: PIO417,
    #[doc = "0x248 - Digital I/O control for port 4 pins PIO4_18"]
    pub pio418: PIO418,
    #[doc = "0x24c - Digital I/O control for port 4 pins PIO4_19"]
    pub pio419: PIO419,
    #[doc = "0x250 - Digital I/O control for port 4 pins PIO4_20"]
    pub pio420: PIO420,
    #[doc = "0x254 - Digital I/O control for port 4 pins PIO4_21"]
    pub pio421: PIO421,
    #[doc = "0x258 - Digital I/O control for port 4 pins PIO4_22"]
    pub pio422: PIO422,
    #[doc = "0x25c - Digital I/O control for port 4 pins PIO4_23"]
    pub pio423: PIO423,
    #[doc = "0x260 - Digital I/O control for port 4 pins PIO4_24"]
    pub pio424: PIO424,
    #[doc = "0x264 - Digital I/O control for port 4 pins PIO4_25"]
    pub pio425: PIO425,
    #[doc = "0x268 - Digital I/O control for port 4 pins PIO4_26"]
    pub pio426: PIO426,
    #[doc = "0x26c - Digital I/O control for port 4 pins PIO4_27"]
    pub pio427: PIO427,
    #[doc = "0x270 - Digital I/O control for port 4 pins PIO4_28"]
    pub pio428: PIO428,
    #[doc = "0x274 - Digital I/O control for port 4 pins PIO4_29"]
    pub pio429: PIO429,
    #[doc = "0x278 - Digital I/O control for port 4 pins PIO4_30"]
    pub pio430: PIO430,
    #[doc = "0x27c - Digital I/O control for port 4 pins PIO4_31"]
    pub pio431: PIO431,
    #[doc = "0x280 - Digital I/O control for port 5 pins PIO5_0"]
    pub pio50: PIO50,
    #[doc = "0x284 - Digital I/O control for port 5 pins PIO5_1"]
    pub pio51: PIO51,
    #[doc = "0x288 - Digital I/O control for port 5 pins PIO5_2"]
    pub pio52: PIO52,
    #[doc = "0x28c - Digital I/O control for port 5 pins PIO5_3"]
    pub pio53: PIO53,
    #[doc = "0x290 - Digital I/O control for port 5 pins PIO5_4"]
    pub pio54: PIO54,
    #[doc = "0x294 - Digital I/O control for port 5 pins PIO5_5"]
    pub pio55: PIO55,
    #[doc = "0x298 - Digital I/O control for port 5 pins PIO5_6"]
    pub pio56: PIO56,
    #[doc = "0x29c - Digital I/O control for port 5 pins PIO5_7"]
    pub pio57: PIO57,
    #[doc = "0x2a0 - Digital I/O control for port 5 pins PIO5_8"]
    pub pio58: PIO58,
    #[doc = "0x2a4 - Digital I/O control for port 5 pins PIO5_9"]
    pub pio59: PIO59,
    #[doc = "0x2a8 - Digital I/O control for port 5 pins PIO5_10"]
    pub pio510: PIO510,
    #[doc = "0x2ac - Digital I/O control for port 5 pins PIO5_11"]
    pub pio511: PIO511,
    #[doc = "0x2b0 - Digital I/O control for port 5 pins PIO5_12"]
    pub pio512: PIO512,
    #[doc = "0x2b4 - Digital I/O control for port 5 pins PIO5_13"]
    pub pio513: PIO513,
    #[doc = "0x2b8 - Digital I/O control for port 5 pins PIO5_14"]
    pub pio514: PIO514,
    #[doc = "0x2bc - Digital I/O control for port 5 pins PIO5_15"]
    pub pio515: PIO515,
    #[doc = "0x2c0 - Digital I/O control for port 5 pins PIO5_16"]
    pub pio516: PIO516,
    #[doc = "0x2c4 - Digital I/O control for port 5 pins PIO5_17"]
    pub pio517: PIO517,
    #[doc = "0x2c8 - Digital I/O control for port 5 pins PIO5_18"]
    pub pio518: PIO518,
    #[doc = "0x2cc - Digital I/O control for port 5 pins PIO5_19"]
    pub pio519: PIO519,
    #[doc = "0x2d0 - Digital I/O control for port 5 pins PIO5_20"]
    pub pio520: PIO520,
    #[doc = "0x2d4 - Digital I/O control for port 5 pins PIO5_21"]
    pub pio521: PIO521,
    #[doc = "0x2d8 - Digital I/O control for port 5 pins PIO5_22"]
    pub pio522: PIO522,
    #[doc = "0x2dc - Digital I/O control for port 5 pins PIO5_23"]
    pub pio523: PIO523,
    #[doc = "0x2e0 - Digital I/O control for port 5 pins PIO5_24"]
    pub pio524: PIO524,
    #[doc = "0x2e4 - Digital I/O control for port 5 pins PIO5_25"]
    pub pio525: PIO525,
    #[doc = "0x2e8 - Digital I/O control for port 5 pins PIO5_26"]
    pub pio526: PIO526,
    #[doc = "0x2ec - Digital I/O control for port 5 pins PIO5_27"]
    pub pio527: PIO527,
    #[doc = "0x2f0 - Digital I/O control for port 5 pins PIO5_28"]
    pub pio528: PIO528,
    #[doc = "0x2f4 - Digital I/O control for port 5 pins PIO5_29"]
    pub pio529: PIO529,
    #[doc = "0x2f8 - Digital I/O control for port 5 pins PIO5_30"]
    pub pio530: PIO530,
    #[doc = "0x2fc - Digital I/O control for port 5 pins PIO5_31"]
    pub pio531: PIO531,
}
#[doc = "Digital I/O control for port 0 pins PIO0_0"]
pub struct PIO00 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_0"]
pub mod pio00;
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
pub struct PIO01 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_1"]
pub mod pio01;
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
pub struct PIO02 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_2"]
pub mod pio02;
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
pub struct PIO03 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_3"]
pub mod pio03;
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
pub struct PIO04 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_4"]
pub mod pio04;
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
pub struct PIO05 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_5"]
pub mod pio05;
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
pub struct PIO06 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_6"]
pub mod pio06;
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
pub struct PIO07 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_7"]
pub mod pio07;
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
pub struct PIO08 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_8"]
pub mod pio08;
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
pub struct PIO09 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_9"]
pub mod pio09;
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
pub struct PIO010 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_10"]
pub mod pio010;
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
pub struct PIO011 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_11"]
pub mod pio011;
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
pub struct PIO012 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_12"]
pub mod pio012;
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
pub struct PIO013 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_13"]
pub mod pio013;
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
pub struct PIO014 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_14"]
pub mod pio014;
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
pub struct PIO015 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_15"]
pub mod pio015;
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
pub struct PIO016 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_16"]
pub mod pio016;
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
pub struct PIO017 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_17"]
pub mod pio017;
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
pub struct PIO018 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_18"]
pub mod pio018;
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
pub struct PIO019 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_19"]
pub mod pio019;
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
pub struct PIO020 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_20"]
pub mod pio020;
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
pub struct PIO021 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_21"]
pub mod pio021;
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
pub struct PIO022 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_22"]
pub mod pio022;
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
pub struct PIO023 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_23"]
pub mod pio023;
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
pub struct PIO024 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_24"]
pub mod pio024;
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
pub struct PIO025 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_25"]
pub mod pio025;
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
pub struct PIO026 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_26"]
pub mod pio026;
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
pub struct PIO027 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_27"]
pub mod pio027;
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
pub struct PIO028 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_28"]
pub mod pio028;
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
pub struct PIO029 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_29"]
pub mod pio029;
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
pub struct PIO030 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_30"]
pub mod pio030;
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
pub struct PIO031 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 0 pins PIO0_31"]
pub mod pio031;
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
pub struct PIO10 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_0"]
pub mod pio10;
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
pub struct PIO11 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_1"]
pub mod pio11;
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
pub struct PIO12 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_2"]
pub mod pio12;
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
pub struct PIO13 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_3"]
pub mod pio13;
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
pub struct PIO14 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_4"]
pub mod pio14;
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
pub struct PIO15 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_5"]
pub mod pio15;
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
pub struct PIO16 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_6"]
pub mod pio16;
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
pub struct PIO17 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_7"]
pub mod pio17;
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
pub struct PIO18 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_8"]
pub mod pio18;
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
pub struct PIO19 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_9"]
pub mod pio19;
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
pub struct PIO110 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_10"]
pub mod pio110;
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
pub struct PIO111 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_11"]
pub mod pio111;
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
pub struct PIO112 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_12"]
pub mod pio112;
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
pub struct PIO113 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_13"]
pub mod pio113;
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
pub struct PIO114 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_14"]
pub mod pio114;
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
pub struct PIO115 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_15"]
pub mod pio115;
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
pub struct PIO116 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_16"]
pub mod pio116;
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
pub struct PIO117 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_17"]
pub mod pio117;
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
pub struct PIO118 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_18"]
pub mod pio118;
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
pub struct PIO119 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_19"]
pub mod pio119;
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
pub struct PIO120 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_20"]
pub mod pio120;
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
pub struct PIO121 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_21"]
pub mod pio121;
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
pub struct PIO122 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_22"]
pub mod pio122;
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
pub struct PIO123 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_23"]
pub mod pio123;
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
pub struct PIO124 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_24"]
pub mod pio124;
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
pub struct PIO125 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_25"]
pub mod pio125;
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
pub struct PIO126 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_26"]
pub mod pio126;
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
pub struct PIO127 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_27"]
pub mod pio127;
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
pub struct PIO128 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_28"]
pub mod pio128;
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
pub struct PIO129 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_29"]
pub mod pio129;
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
pub struct PIO130 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_30"]
pub mod pio130;
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
pub struct PIO131 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 1 pins PIO1_31"]
pub mod pio131;
#[doc = "Digital I/O control for port 2 pins PIO2_0"]
pub struct PIO20 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_0"]
pub mod pio20;
#[doc = "Digital I/O control for port 2 pins PIO2_1"]
pub struct PIO21 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_1"]
pub mod pio21;
#[doc = "Digital I/O control for port 2 pins PIO2_2"]
pub struct PIO22 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_2"]
pub mod pio22;
#[doc = "Digital I/O control for port 2 pins PIO2_3"]
pub struct PIO23 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_3"]
pub mod pio23;
#[doc = "Digital I/O control for port 2 pins PIO2_4"]
pub struct PIO24 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_4"]
pub mod pio24;
#[doc = "Digital I/O control for port 2 pins PIO2_5"]
pub struct PIO25 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_5"]
pub mod pio25;
#[doc = "Digital I/O control for port 2 pins PIO2_6"]
pub struct PIO26 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_6"]
pub mod pio26;
#[doc = "Digital I/O control for port 2 pins PIO2_7"]
pub struct PIO27 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_7"]
pub mod pio27;
#[doc = "Digital I/O control for port 2 pins PIO2_8"]
pub struct PIO28 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_8"]
pub mod pio28;
#[doc = "Digital I/O control for port 2 pins PIO2_9"]
pub struct PIO29 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_9"]
pub mod pio29;
#[doc = "Digital I/O control for port 2 pins PIO2_10"]
pub struct PIO210 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_10"]
pub mod pio210;
#[doc = "Digital I/O control for port 2 pins PIO2_11"]
pub struct PIO211 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_11"]
pub mod pio211;
#[doc = "Digital I/O control for port 2 pins PIO2_12"]
pub struct PIO212 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_12"]
pub mod pio212;
#[doc = "Digital I/O control for port 2 pins PIO2_13"]
pub struct PIO213 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_13"]
pub mod pio213;
#[doc = "Digital I/O control for port 2 pins PIO2_14"]
pub struct PIO214 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_14"]
pub mod pio214;
#[doc = "Digital I/O control for port 2 pins PIO2_15"]
pub struct PIO215 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_15"]
pub mod pio215;
#[doc = "Digital I/O control for port 2 pins PIO2_16"]
pub struct PIO216 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_16"]
pub mod pio216;
#[doc = "Digital I/O control for port 2 pins PIO2_17"]
pub struct PIO217 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_17"]
pub mod pio217;
#[doc = "Digital I/O control for port 2 pins PIO2_18"]
pub struct PIO218 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_18"]
pub mod pio218;
#[doc = "Digital I/O control for port 2 pins PIO2_19"]
pub struct PIO219 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_19"]
pub mod pio219;
#[doc = "Digital I/O control for port 2 pins PIO2_20"]
pub struct PIO220 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_20"]
pub mod pio220;
#[doc = "Digital I/O control for port 2 pins PIO2_21"]
pub struct PIO221 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_21"]
pub mod pio221;
#[doc = "Digital I/O control for port 2 pins PIO2_22"]
pub struct PIO222 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_22"]
pub mod pio222;
#[doc = "Digital I/O control for port 2 pins PIO2_23"]
pub struct PIO223 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_23"]
pub mod pio223;
#[doc = "Digital I/O control for port 2 pins PIO2_24"]
pub struct PIO224 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_24"]
pub mod pio224;
#[doc = "Digital I/O control for port 2 pins PIO2_25"]
pub struct PIO225 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_25"]
pub mod pio225;
#[doc = "Digital I/O control for port 2 pins PIO2_26"]
pub struct PIO226 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_26"]
pub mod pio226;
#[doc = "Digital I/O control for port 2 pins PIO2_27"]
pub struct PIO227 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_27"]
pub mod pio227;
#[doc = "Digital I/O control for port 2 pins PIO2_28"]
pub struct PIO228 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_28"]
pub mod pio228;
#[doc = "Digital I/O control for port 2 pins PIO2_29"]
pub struct PIO229 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_29"]
pub mod pio229;
#[doc = "Digital I/O control for port 2 pins PIO2_30"]
pub struct PIO230 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_30"]
pub mod pio230;
#[doc = "Digital I/O control for port 2 pins PIO2_31"]
pub struct PIO231 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 2 pins PIO2_31"]
pub mod pio231;
#[doc = "Digital I/O control for port 3 pins PIO3_0"]
pub struct PIO30 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_0"]
pub mod pio30;
#[doc = "Digital I/O control for port 3 pins PIO3_1"]
pub struct PIO31 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_1"]
pub mod pio31;
#[doc = "Digital I/O control for port 3 pins PIO3_2"]
pub struct PIO32 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_2"]
pub mod pio32;
#[doc = "Digital I/O control for port 3 pins PIO3_3"]
pub struct PIO33 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_3"]
pub mod pio33;
#[doc = "Digital I/O control for port 3 pins PIO3_4"]
pub struct PIO34 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_4"]
pub mod pio34;
#[doc = "Digital I/O control for port 3 pins PIO3_5"]
pub struct PIO35 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_5"]
pub mod pio35;
#[doc = "Digital I/O control for port 3 pins PIO3_6"]
pub struct PIO36 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_6"]
pub mod pio36;
#[doc = "Digital I/O control for port 3 pins PIO3_7"]
pub struct PIO37 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_7"]
pub mod pio37;
#[doc = "Digital I/O control for port 3 pins PIO3_8"]
pub struct PIO38 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_8"]
pub mod pio38;
#[doc = "Digital I/O control for port 3 pins PIO3_9"]
pub struct PIO39 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_9"]
pub mod pio39;
#[doc = "Digital I/O control for port 3 pins PIO3_10"]
pub struct PIO310 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_10"]
pub mod pio310;
#[doc = "Digital I/O control for port 3 pins PIO3_11"]
pub struct PIO311 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_11"]
pub mod pio311;
#[doc = "Digital I/O control for port 3 pins PIO3_12"]
pub struct PIO312 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_12"]
pub mod pio312;
#[doc = "Digital I/O control for port 3 pins PIO3_13"]
pub struct PIO313 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_13"]
pub mod pio313;
#[doc = "Digital I/O control for port 3 pins PIO3_14"]
pub struct PIO314 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_14"]
pub mod pio314;
#[doc = "Digital I/O control for port 3 pins PIO3_15"]
pub struct PIO315 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_15"]
pub mod pio315;
#[doc = "Digital I/O control for port 3 pins PIO3_16"]
pub struct PIO316 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_16"]
pub mod pio316;
#[doc = "Digital I/O control for port 3 pins PIO3_17"]
pub struct PIO317 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_17"]
pub mod pio317;
#[doc = "Digital I/O control for port 3 pins PIO3_18"]
pub struct PIO318 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_18"]
pub mod pio318;
#[doc = "Digital I/O control for port 3 pins PIO3_19"]
pub struct PIO319 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_19"]
pub mod pio319;
#[doc = "Digital I/O control for port 3 pins PIO3_20"]
pub struct PIO320 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_20"]
pub mod pio320;
#[doc = "Digital I/O control for port 3 pins PIO3_21"]
pub struct PIO321 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_21"]
pub mod pio321;
#[doc = "Digital I/O control for port 3 pins PIO3_22"]
pub struct PIO322 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_22"]
pub mod pio322;
#[doc = "Digital I/O control for port 3 pins PIO3_23"]
pub struct PIO323 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_23"]
pub mod pio323;
#[doc = "Digital I/O control for port 3 pins PIO3_24"]
pub struct PIO324 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_24"]
pub mod pio324;
#[doc = "Digital I/O control for port 3 pins PIO3_25"]
pub struct PIO325 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_25"]
pub mod pio325;
#[doc = "Digital I/O control for port 3 pins PIO3_26"]
pub struct PIO326 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_26"]
pub mod pio326;
#[doc = "Digital I/O control for port 3 pins PIO3_27"]
pub struct PIO327 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_27"]
pub mod pio327;
#[doc = "Digital I/O control for port 3 pins PIO3_28"]
pub struct PIO328 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_28"]
pub mod pio328;
#[doc = "Digital I/O control for port 3 pins PIO3_29"]
pub struct PIO329 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_29"]
pub mod pio329;
#[doc = "Digital I/O control for port 3 pins PIO3_30"]
pub struct PIO330 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_30"]
pub mod pio330;
#[doc = "Digital I/O control for port 3 pins PIO3_31"]
pub struct PIO331 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 3 pins PIO3_31"]
pub mod pio331;
#[doc = "Digital I/O control for port 4 pins PIO4_0"]
pub struct PIO40 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_0"]
pub mod pio40;
#[doc = "Digital I/O control for port 4 pins PIO4_1"]
pub struct PIO41 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_1"]
pub mod pio41;
#[doc = "Digital I/O control for port 4 pins PIO4_2"]
pub struct PIO42 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_2"]
pub mod pio42;
#[doc = "Digital I/O control for port 4 pins PIO4_3"]
pub struct PIO43 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_3"]
pub mod pio43;
#[doc = "Digital I/O control for port 4 pins PIO4_4"]
pub struct PIO44 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_4"]
pub mod pio44;
#[doc = "Digital I/O control for port 4 pins PIO4_5"]
pub struct PIO45 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_5"]
pub mod pio45;
#[doc = "Digital I/O control for port 4 pins PIO4_6"]
pub struct PIO46 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_6"]
pub mod pio46;
#[doc = "Digital I/O control for port 4 pins PIO4_7"]
pub struct PIO47 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_7"]
pub mod pio47;
#[doc = "Digital I/O control for port 4 pins PIO4_8"]
pub struct PIO48 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_8"]
pub mod pio48;
#[doc = "Digital I/O control for port 4 pins PIO4_9"]
pub struct PIO49 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_9"]
pub mod pio49;
#[doc = "Digital I/O control for port 4 pins PIO4_10"]
pub struct PIO410 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_10"]
pub mod pio410;
#[doc = "Digital I/O control for port 4 pins PIO4_11"]
pub struct PIO411 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_11"]
pub mod pio411;
#[doc = "Digital I/O control for port 4 pins PIO4_12"]
pub struct PIO412 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_12"]
pub mod pio412;
#[doc = "Digital I/O control for port 4 pins PIO4_13"]
pub struct PIO413 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_13"]
pub mod pio413;
#[doc = "Digital I/O control for port 4 pins PIO4_14"]
pub struct PIO414 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_14"]
pub mod pio414;
#[doc = "Digital I/O control for port 4 pins PIO4_15"]
pub struct PIO415 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_15"]
pub mod pio415;
#[doc = "Digital I/O control for port 4 pins PIO4_16"]
pub struct PIO416 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_16"]
pub mod pio416;
#[doc = "Digital I/O control for port 4 pins PIO4_17"]
pub struct PIO417 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_17"]
pub mod pio417;
#[doc = "Digital I/O control for port 4 pins PIO4_18"]
pub struct PIO418 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_18"]
pub mod pio418;
#[doc = "Digital I/O control for port 4 pins PIO4_19"]
pub struct PIO419 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_19"]
pub mod pio419;
#[doc = "Digital I/O control for port 4 pins PIO4_20"]
pub struct PIO420 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_20"]
pub mod pio420;
#[doc = "Digital I/O control for port 4 pins PIO4_21"]
pub struct PIO421 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_21"]
pub mod pio421;
#[doc = "Digital I/O control for port 4 pins PIO4_22"]
pub struct PIO422 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_22"]
pub mod pio422;
#[doc = "Digital I/O control for port 4 pins PIO4_23"]
pub struct PIO423 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_23"]
pub mod pio423;
#[doc = "Digital I/O control for port 4 pins PIO4_24"]
pub struct PIO424 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_24"]
pub mod pio424;
#[doc = "Digital I/O control for port 4 pins PIO4_25"]
pub struct PIO425 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_25"]
pub mod pio425;
#[doc = "Digital I/O control for port 4 pins PIO4_26"]
pub struct PIO426 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_26"]
pub mod pio426;
#[doc = "Digital I/O control for port 4 pins PIO4_27"]
pub struct PIO427 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_27"]
pub mod pio427;
#[doc = "Digital I/O control for port 4 pins PIO4_28"]
pub struct PIO428 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_28"]
pub mod pio428;
#[doc = "Digital I/O control for port 4 pins PIO4_29"]
pub struct PIO429 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_29"]
pub mod pio429;
#[doc = "Digital I/O control for port 4 pins PIO4_30"]
pub struct PIO430 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_30"]
pub mod pio430;
#[doc = "Digital I/O control for port 4 pins PIO4_31"]
pub struct PIO431 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 4 pins PIO4_31"]
pub mod pio431;
#[doc = "Digital I/O control for port 5 pins PIO5_0"]
pub struct PIO50 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_0"]
pub mod pio50;
#[doc = "Digital I/O control for port 5 pins PIO5_1"]
pub struct PIO51 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_1"]
pub mod pio51;
#[doc = "Digital I/O control for port 5 pins PIO5_2"]
pub struct PIO52 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_2"]
pub mod pio52;
#[doc = "Digital I/O control for port 5 pins PIO5_3"]
pub struct PIO53 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_3"]
pub mod pio53;
#[doc = "Digital I/O control for port 5 pins PIO5_4"]
pub struct PIO54 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_4"]
pub mod pio54;
#[doc = "Digital I/O control for port 5 pins PIO5_5"]
pub struct PIO55 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_5"]
pub mod pio55;
#[doc = "Digital I/O control for port 5 pins PIO5_6"]
pub struct PIO56 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_6"]
pub mod pio56;
#[doc = "Digital I/O control for port 5 pins PIO5_7"]
pub struct PIO57 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_7"]
pub mod pio57;
#[doc = "Digital I/O control for port 5 pins PIO5_8"]
pub struct PIO58 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_8"]
pub mod pio58;
#[doc = "Digital I/O control for port 5 pins PIO5_9"]
pub struct PIO59 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_9"]
pub mod pio59;
#[doc = "Digital I/O control for port 5 pins PIO5_10"]
pub struct PIO510 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_10"]
pub mod pio510;
#[doc = "Digital I/O control for port 5 pins PIO5_11"]
pub struct PIO511 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_11"]
pub mod pio511;
#[doc = "Digital I/O control for port 5 pins PIO5_12"]
pub struct PIO512 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_12"]
pub mod pio512;
#[doc = "Digital I/O control for port 5 pins PIO5_13"]
pub struct PIO513 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_13"]
pub mod pio513;
#[doc = "Digital I/O control for port 5 pins PIO5_14"]
pub struct PIO514 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_14"]
pub mod pio514;
#[doc = "Digital I/O control for port 5 pins PIO5_15"]
pub struct PIO515 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_15"]
pub mod pio515;
#[doc = "Digital I/O control for port 5 pins PIO5_16"]
pub struct PIO516 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_16"]
pub mod pio516;
#[doc = "Digital I/O control for port 5 pins PIO5_17"]
pub struct PIO517 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_17"]
pub mod pio517;
#[doc = "Digital I/O control for port 5 pins PIO5_18"]
pub struct PIO518 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_18"]
pub mod pio518;
#[doc = "Digital I/O control for port 5 pins PIO5_19"]
pub struct PIO519 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_19"]
pub mod pio519;
#[doc = "Digital I/O control for port 5 pins PIO5_20"]
pub struct PIO520 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_20"]
pub mod pio520;
#[doc = "Digital I/O control for port 5 pins PIO5_21"]
pub struct PIO521 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_21"]
pub mod pio521;
#[doc = "Digital I/O control for port 5 pins PIO5_22"]
pub struct PIO522 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_22"]
pub mod pio522;
#[doc = "Digital I/O control for port 5 pins PIO5_23"]
pub struct PIO523 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_23"]
pub mod pio523;
#[doc = "Digital I/O control for port 5 pins PIO5_24"]
pub struct PIO524 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_24"]
pub mod pio524;
#[doc = "Digital I/O control for port 5 pins PIO5_25"]
pub struct PIO525 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_25"]
pub mod pio525;
#[doc = "Digital I/O control for port 5 pins PIO5_26"]
pub struct PIO526 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_26"]
pub mod pio526;
#[doc = "Digital I/O control for port 5 pins PIO5_27"]
pub struct PIO527 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_27"]
pub mod pio527;
#[doc = "Digital I/O control for port 5 pins PIO5_28"]
pub struct PIO528 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_28"]
pub mod pio528;
#[doc = "Digital I/O control for port 5 pins PIO5_29"]
pub struct PIO529 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_29"]
pub mod pio529;
#[doc = "Digital I/O control for port 5 pins PIO5_30"]
pub struct PIO530 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_30"]
pub mod pio530;
#[doc = "Digital I/O control for port 5 pins PIO5_31"]
pub struct PIO531 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital I/O control for port 5 pins PIO5_31"]
pub mod pio531;
