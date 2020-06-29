use wasm_bindgen::prelude::*;
use std::mem;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Hemisphere {
	Northern = 0, Southern = 1
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Weather {
	Clear = 0, Sunny = 1, Cloudy = 2, RainClouds = 3, Rain = 4, HeavyRain = 5
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum WindType {
	Calm = 0, Land0 = 1, Land1 = 2, Land2 = 3, Sea0 = 4, Sea1 = 5, Sea2 = 6
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SpecialDay {
	None, Easter, FishCon, InsectCon, Countdown
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SnowLevel {
	None, Low, Full
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CloudLevel {
	None, Cumulonimbus, Cirrus, Billow, Thin
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SpWeatherLevel {
	None, Rainbow, Aurora
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FogLevel {
	None, HeavyAndWater, WaterOnly
}
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Constellation {
	Capricorn, Aquarius, Pisces, Aries, Taurus, Gemini, Cancer, Leo, Virgo, Libra, Scorpio, Sagittarius
}

pub static EASTER_DAYS: [u8;61] = [23,15,31,20,11,27,16,8,23,12,4,24,8,31,20,5,27,16,1,21,12,4,17,9,31,20,5,28,16,1,21,13,28,17,9,25,13,5,25,10,1,21,6,29,17,9,25,14,5,18,10,2,21,6,29,18,2,22,14,30,18];
pub static EASTER_MONTHS: [u8;61] = [4,4,3,4,4,3,4,4,3,4,4,4,4,3,4,4,3,4,4,4,4,4,4,4,3,4,4,3,4,4,4,4,3,4,4,3,4,4,4,4,4,4,4,3,4,4,3,4,4,4,4,4,4,4,3,4,4,4,4,3,4];

pub static FISH_CON_JAN: [u8;61] = [8,13,12,11,10,8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10];
pub static FISH_CON_APR: [u8;61] = [8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10];
pub static FISH_CON_JUL: [u8;61] = [8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10];
pub static FISH_CON_OCT: [u8;61] = [14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9,8,14,13,11,10,9,8,13,12,11,10,8,14,13,12,10,9,8,14,12,11,10,9,14,13,12,11,9];
pub static INSECT_CON_JUN_N: [u8;61] = [24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26];
pub static INSECT_CON_JUL_N: [u8;61] = [22,28,27,26,24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24];
pub static INSECT_CON_AUG_N: [u8;61] = [26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26,25,24,23,28];
pub static INSECT_CON_SEP_N: [u8;61] = [23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25,24,23,22,27,26,25,24,22,28,27,26,24,23,22,28,26,25,24,23,28,27,26,25,23,22,28,27,25];
pub static INSECT_CON_JAN_S: [u8;61] = [15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20,19,18,17];
pub static INSECT_CON_FEB_S: [u8;61] = [19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21];
pub static INSECT_CON_MAR_S: [u8;61] = [18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20];
pub static INSECT_CON_DEC_S: [u8;61] = [16,15,21,20,18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18,17,16,15,20,19,18,17,15,21,20,19,17,16,15,21,19,18,17,16,21,20,19,18,16,15,21,20,18];

pub static RATE_LOOKUP_N: [[u8;31];12] = [
	[0,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],
	[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,4,4,4,4,4,4,4],
	[4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4],
	[5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6],
	[6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6],
	[6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7],
	[7,7,7,7,7,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8],
	[9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9],
	[10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12],
	[12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12],
	[12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,13,13,13,13,13,13,13,13,14,15,15,15,15,15,15],
	[15,15,15,15,15,15,15,15,15,16,17,17,17,17,17,17,17,17,17,17,17,17,17,18,18,18,18,18,18,18,19],
];
pub static RATE_LOOKUP_S: [[u8;31];12] = [
	[20,21,21,21,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22],
	[23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,23,24,24],
	[24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25,25],
	[26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26],
	[26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,27,27,27,27,27,27,27,27,27,28,29,29,29,29,29,29],
	[29,29,29,29,29,29,29,29,29,30,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,32],
	[32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32],
	[32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,33,34,34,34,34,34,34,34],
	[34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,35],
	[35,35,35,35,35,35,35,35,35,35,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36],
	[36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,36],
	[36,36,36,36,36,36,36,36,36,36,36,36,36,36,36,37,37,37,37,37,37,37,37,38,38,38,38,38,38,38,39],
];
pub static RATE_MAPS: [[u8;100];40] = [
	[0,0,0,0,0,0,0,0,0,0,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,],
	[0,0,0,0,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,8,9,10,10,10,11,11,11,12,12,12,13,13,13,14,14,14,15,15,15,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,19,19,19,19,19,19,19,19,19,19,20,20,20,20,20,20,20,20,20,20,21,21,21,21,21,21,21,21,21,21,],
	[0,0,0,0,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,8,8,8,9,9,9,10,10,11,11,12,12,16,16,17,17,18,18,19,19,20,20,21,21,26,26,26,27,27,27,28,28,28,29,29,30,30,31,31,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,],
	[0,0,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,11,11,11,12,12,12,13,13,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[0,0,2,2,2,2,2,2,2,2,4,4,4,4,4,4,4,4,6,6,6,6,6,6,6,6,7,7,7,8,8,8,9,9,9,9,9,10,10,10,10,10,10,10,10,10,10,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,13,13,13,14,14,14,15,15,15,16,17,18,19,20,21,26,26,26,27,27,27,28,28,28,29,29,29,30,30,30,31,31,31,],
	[0,0,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,8,9,9,10,11,12,13,14,15,16,16,17,17,18,18,19,19,20,20,21,21,22,22,22,22,22,23,23,23,23,23,24,24,24,25,25,25,],
	[0,0,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,8,9,19,19,20,20,21,21,22,22,22,23,23,23,24,24,24,24,24,24,24,24,24,24,25,25,25,25,25,25,25,25,25,25,],
	[0,0,0,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,10,10,11,11,12,12,13,13,13,14,14,14,15,15,15,22,22,22,22,22,22,22,22,23,23,23,23,23,23,23,23,24,24,24,24,24,24,25,25,25,25,25,25,26,27,28,29,30,31,],
	[0,0,0,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,8,8,9,10,10,11,11,12,12,13,13,14,14,15,15,16,17,18,19,19,20,20,21,21,22,22,22,22,23,23,23,23,26,27,28,29,30,31,],
	[0,0,0,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,14,15,16,16,17,17,17,17,18,18,18,18,19,19,20,20,21,21,22,22,22,23,23,23,],
	[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,8,8,8,8,8,8,8,],
	[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,7,7,7,7,8,8,8,8,],
	[0,0,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,7,7,7,8,8,8,9,9,9,10,10,10,11,11,11,12,12,12,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,],
	[0,0,0,0,1,1,1,2,2,2,2,2,2,2,2,2,3,3,3,4,4,4,4,4,4,4,4,4,5,5,5,6,6,6,6,6,6,6,6,6,7,7,8,8,9,9,10,10,10,10,10,11,11,11,11,11,12,12,12,12,12,13,13,13,14,14,14,15,15,15,16,16,17,17,18,18,19,19,20,20,21,21,26,26,26,27,27,27,28,28,28,29,29,29,30,30,30,31,31,31,],
	[10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,13,13,13,13,13,13,13,13,13,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,],

	[0,0,0,0,0,0,0,0,0,0,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,],
	[0,0,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,8,9,9,10,11,12,13,14,15,16,16,17,17,18,18,19,19,20,20,21,21,22,22,22,22,22,23,23,23,23,23,24,24,24,25,25,25,],
	[0,0,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,8,9,10,11,12,13,14,15,16,17,19,19,20,20,21,21,22,22,22,22,23,23,23,23,24,24,24,24,24,25,25,25,25,25,],
	[0,0,0,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13,13,14,14,14,15,15,15,16,17,18,19,20,21,22,22,22,22,23,23,23,23,24,24,24,24,25,25,25,25,26,27,28,29,30,31,],
	[0,0,0,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,8,8,9,10,10,11,11,12,12,13,13,14,14,15,15,16,17,18,19,19,20,20,21,21,22,22,22,22,23,23,23,23,26,27,28,29,30,31,],
	[0,0,0,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,8,8,8,9,9,9,10,10,11,11,12,12,13,14,15,16,16,17,17,18,18,19,19,20,20,21,21,22,22,22,23,23,23,26,27,29,30,],
	[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,8,8,8,16,16,17,17,18,18,19,19,20,20,21,21,],
	[0,0,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,19,19,19,19,19,19,19,19,19,19,20,20,20,20,20,20,20,20,20,20,21,21,21,21,21,21,21,21,21,21,],
	[0,0,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,7,7,7,8,8,8,9,9,9,10,10,10,11,11,11,12,12,12,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,],
	[0,0,0,0,1,1,1,2,2,2,2,2,2,2,2,2,3,3,3,4,4,4,4,4,4,4,4,4,5,5,5,6,6,6,6,6,6,6,6,6,7,7,8,8,9,9,10,10,10,10,10,11,11,11,11,11,12,12,12,12,12,13,13,13,14,14,14,15,15,15,16,16,17,17,18,18,19,19,20,20,21,21,26,26,26,27,27,27,28,28,28,29,29,29,30,30,30,31,31,31,],
	[0,0,0,0,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,8,9,10,10,10,11,11,11,12,12,12,13,13,13,14,14,14,15,15,15,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,19,19,19,19,19,19,19,19,19,19,20,20,20,20,20,20,20,20,20,20,21,21,21,21,21,21,21,21,21,21,],
	[0,0,0,0,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,8,8,8,9,9,9,10,10,11,11,12,12,16,16,17,17,18,18,19,19,20,20,21,21,26,26,26,27,27,27,28,28,28,29,29,30,30,31,31,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,],
	[0,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,7,7,7,8,8,8,8,8,8,9,9,9,9,9,9,10,10,10,11,11,11,12,12,12,26,26,27,27,28,28,29,29,30,30,31,31,],
	[0,0,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,11,11,11,12,12,12,13,13,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[0,0,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,11,11,11,12,12,12,13,13,16,16,17,17,18,18,19,19,20,20,21,21,26,26,27,27,28,28,29,29,30,30,31,31,],
	[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,],
];



use Weather::Clear as F;
use Weather::Sunny as C;
use Weather::Cloudy as O;
use Weather::RainClouds as RC;
use Weather::Rain as R;
use Weather::HeavyRain as HR;

pub static PATTERNS: [[Weather;24];34] = [
//   0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23
	[F, F, C, C, F, F, C, F, C, F, C, C, F, C, F, C, C, F, C, F, F, F, C, F,],  // Fine00
	[F, F, C, F, F, C, F, C, C, C, F, C, O, F, C, F, C, C, O, C, O, C, F, C,],  // Fine01
	[F, F, C, C, C, C, C, F, C, F, O, F, C, F, C, C, F, C, C, O, C, C, F, F,],  // Fine02
	[F, C, F, F, F, C, C, F, C, C, F, C, C, O, C, F, C, C, C, F, O, F, F, C,],  // Fine03
	[F, F, F, C, C, C, C, C, C, O, C, C, F, C, O, C, F, C, F, C, C, F, C, F,],  // Fine04
	[O, C, C, F, C, C, C, C, F, C, C, F, C, C, O, C, C, F, C, O, C, C, F, C,],  // Fine05
	[F, C, F, C, C, C, O, C, C, O, F, C, F, C, C, F, C, C, O, O, C, C, F, F,],  // Fine06
	[C, C, C, O, O, O, RC,O, O, O, C, C, O, O, O, C, C, O, O, O, RC,O, O, C,],  // Cloud00
	[C, O, RC,O, O, O, O, O, C, O, RC,O, O, C, O, O, O, RC,RC,O, O, C, C, C,],  // Cloud01
	[O, O, O, RC,O, O, O, RC,R, R, R, O, O, RC,R, R, R, O, O, RC,O, RC,O, O,],  // Cloud02
	[C, O, RC,R, R, R, R, RC,O, C, RC,R, R, R, R, R, R, O, O, RC,R, R, R, O,],  // Rain00
	[RC,R, R, R, R, R, R, R, R, O, RC,R, R, O, O, RC,HR,R, R, R, R, C, C, O,],  // Rain01
	[RC,O, R, R, R, R, R, R, RC,R, C, RC,R, R, R, O, R, R, R, RC,HR,R, R, R,],  // Rain02
	[R, R, R, R, R, R, RC,R, R, HR,HR,R, R, HR,HR,HR,HR,R, R, R, R, R, O, RC,], // Rain03
	[R, R, HR,HR,R, R, R, HR,HR,HR,HR,R, HR,HR,HR,R, R, HR,HR,HR,HR,HR,HR,R,],  // Rain04
	[RC,R, R, HR,R, HR,HR,R, R, HR,HR,HR,HR,R, HR,HR,HR,HR,HR,HR,R, R, O, C,],  // Rain05
//   0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23
	[RC,O, RC,RC,O, R, O, O, O, O, O, C, C, F, C, C, F, C, C, C, O, O, O, O,],  // FineCloud00
	[O, RC,O, O, O, O, C, C, C, F, C, C, F, C, C, O, RC,R, R, O, O, RC,O, O,],  // FineCloud01
	[R, R, O, O, O, O, F, C, F, C, C, O, O, C, O, O, O, O, O, RC,O, RC,RC,R,],  // FineCloud02
	[F, C, C, C, C, RC,R, R, O, R, C, O, C, O, O, C, C, F, C, C, C, C, F, F,],  // CloudFine00
	[C, F, F, C, C, C, R, O, O, O, O, RC,R, C, C, O, C, C, F, C, C, F, C, C,],  // CloudFine01
	[C, C, C, C, C, F, O, RC,R, O, RC,R, C, C, C, C, F, C, F, C, F, F, C, F,],  // CloudFine02
	[F, F, C, C, C, C, C, F, C, F, C, F, C, R, C, C, C, C, C, F, C, F, C, C,],  // FineRain00
	[F, C, F, C, C, C, F, C, C, F, C, C, R, O, R, C, C, C, F, C, C, C, C, C,],  // FineRain01
	[C, F, F, C, C, F, F, C, F, C, F, C, F, C, C, HR,O, C, F, C, C, F, C, F,],  // FineRain02
	[F, F, C, F, C, C, C, C, F, C, F, F, C, C, HR,R, C, C, C, F, C, C, F, C,],  // FineRain03
	[R, R, R, R, R, R, O, O, C, O, O, RC,R, R, R, HR,HR,R, R, R, R, R, R, R,],  // CloudRain00
	[R, R, R, R, O, O, C, C, O, O, R, R, R, O, O, RC,R, R, R, R, R, O, R, R,],  // CloudRain01
	[HR,HR,R, HR,R, R, O, O, O, C, C, O, O, RC,R, R, R, R, R, R, R, R, R, RC,], // CloudRain02
	[O, RC,O, O, O, O, R, R, O, O, RC,R, R, O, RC,O, C, C, O, O, O, O, O, O,],  // RainCloud00
	[O, O, O, O, O, RC,R, R, R, R, R, R, R, R, R, O, RC,O, O, O, O, O, C, C,],  // RainCloud01
	[C, C, O, O, RC,RC,HR,HR,R, R, R, O, O, R, O, O, O, O, O, RC,O, RC,O, O,],  // RainCloud02
//   0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23
	[F, F, C, F, F, C, F, C, C, C, F, C, O, F, C, F, C, C, O, C, O, C, F, C,],  // Commun00
	[F, F, C, F, F, C, F, C, C, C, F, C, C, F, C, F, C, C, F, C, F, C, F, C,],  // EventDay00
];

use WindType::Calm as WC;
use WindType::Land0 as WL0;
use WindType::Land1 as WL1;
use WindType::Land2 as WL2;
use WindType::Sea0 as WS0;
use WindType::Sea1 as WS1;
use WindType::Sea2 as WS2;

pub static WINDS: [[WindType;24];34] = [
	[WL0,WL1,WL1,WL2,WL2,WL0,WL0,WC,WS0,WS0,WS0,WS1,WS0,WS1,WS2,WS2,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL0,], // Fine00
	[WL1,WL0,WL1,WL1,WL1,WL1,WL0,WC,WS0,WS0,WS0,WS1,WS2,WS0,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL0,WL0,], // Fine01
	[WL1,WL0,WL1,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS2,WS1,WS1,WS0,WS0,WS0,WC,WL0,WL0,WL1,WL2,], // Fine02
	[WL1,WL2,WL1,WL1,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL0,], // Fine03
	[WL2,WL1,WL0,WL1,WL2,WL1,WL0,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS1,WS2,WS0,WS0,WC,WL0,WL0,WL0,WL1,WL1,], // Fine04
	[WL0,WL1,WL2,WL1,WL1,WL1,WL0,WC,WS0,WS0,WS1,WS2,WS1,WS0,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL0,WL0,WL1,], // Fine05
	[WL0,WL1,WL1,WL2,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS2,WS2,WS1,WS0,WC,WL0,WL0,WL1,WL1,WL0,], // Fine06
	[WL1,WL0,WL1,WL2,WL2,WL1,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS1,WS0,WS1,WS0,WC,WL0,WL0,WL1,WL0,WL0,], // Cloud00
	[WL1,WL2,WL2,WL1,WL2,WL0,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS1,WS0,WS0,WS0,WS0,WC,WL0,WL0,WL0,WL1,], // Cloud01
	[WL1,WL1,WL2,WL2,WL1,WL1,WC,WS0,WS0,WS1,WS1,WS1,WS0,WS0,WS0,WS1,WS1,WS0,WC,WL0,WL0,WL1,WL0,WL0,], // Cloud02
	[WL2,WL1,WL0,WL1,WL2,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS1,WS2,WS1,WS0,WS1,WS1,WS0,WC,WL0,WL0,WL1,WL1,], // Rain00
	[WL0,WL1,WL1,WL2,WL2,WL1,WC,WS0,WS0,WS1,WS1,WS2,WS1,WS2,WS1,WS0,WS1,WS0,WS0,WC,WL0,WL1,WL1,WL0,], // Rain01
	[WL2,WL1,WL0,WL1,WL2,WL2,WL0,WC,WS0,WS1,WS2,WS2,WS1,WS2,WS1,WS2,WS1,WS0,WC,WL0,WL1,WL1,WL0,WL1,], // Rain02
	[WL1,WL2,WL1,WL0,WL1,WL1,WC,WS0,WS0,WS1,WS1,WS2,WS1,WS1,WS2,WS2,WS1,WS0,WS1,WC,WL0,WL1,WL1,WL0,], // Rain03
	[WL1,WL0,WL0,WL1,WL1,WL2,WL1,WC,WS1,WS0,WS1,WS0,WS1,WS2,WS1,WS2,WS1,WS0,WC,WL1,WL1,WL2,WL1,WL2,], // Rain04
	[WL0,WL0,WL1,WL2,WL1,WL0,WC,WS0,WS1,WS2,WS1,WS1,WS0,WS1,WS2,WS1,WS0,WS1,WS0,WC,WL0,WL1,WL2,WL1,], // Rain05
	[WL0,WL2,WL1,WL1,WL2,WL0,WC,WS0,WS0,WS0,WS1,WS0,WS1,WS2,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL1,], // FineCloud00
	[WL2,WL1,WL2,WL1,WL1,WL1,WL0,WC,WS0,WS0,WS1,WS1,WS2,WS1,WS0,WS2,WS1,WS0,WS0,WC,WL0,WL1,WL1,WL0,], // FineCloud01
	[WL1,WL0,WL2,WL1,WL2,WL0,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS2,WS2,WS1,WS0,WC,WL0,WL0,WL1,WL1,WL2,], // FineCloud02
	[WL1,WL2,WL2,WL1,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS0,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL0,WL1,], // CloudFine00
	[WL0,WL0,WL1,WL1,WL2,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL1,WL1,WL0,], // CloudFine01
	[WL1,WL0,WL1,WL2,WL1,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS2,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL2,], // CloudFine02
	[WL2,WL1,WL0,WL1,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS2,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL1,WL1,], // FineRain00
	[WL0,WL1,WL2,WL0,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS1,WS2,WS2,WS1,WS1,WS0,WC,WL0,WL0,WL1,WL0,WL0,], // FineRain01
	[WL1,WL2,WL1,WL1,WL2,WL1,WL0,WC,WS0,WS0,WS0,WS1,WS1,WS2,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL0,WL1,], // FineRain02
	[WL0,WL1,WL2,WL2,WL2,WL1,WL0,WL0,WC,WS0,WS0,WS0,WS1,WS1,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL1,WL0,WL0,], // FineRain03
	[WL1,WL0,WL0,WL1,WL2,WL1,WC,WS0,WS0,WS0,WS1,WS2,WS1,WS1,WS2,WS1,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL2,], // CloudRain00
	[WL1,WL2,WL1,WL2,WL1,WL1,WL0,WC,WS0,WS0,WS0,WS0,WS1,WS2,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL1,WL0,WL1,], // CloudRain01
	[WL2,WL1,WL0,WL1,WL2,WL1,WL0,WC,WS0,WS0,WS1,WS0,WS0,WS1,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL1,WL1,WL2,], // CloudRain02
	[WL2,WL1,WL0,WL1,WL2,WL1,WC,WS0,WS0,WS0,WS1,WS0,WS0,WS1,WS1,WS2,WS1,WS0,WC,WL0,WL0,WL1,WL0,WL1,], // RainCloud00
	[WL1,WL2,WL1,WL2,WL2,WL1,WL0,WC,WS0,WS0,WS0,WS1,WS0,WS1,WS2,WS1,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL1,], // RainCloud01
	[WL1,WL0,WL1,WL0,WL2,WS0,WC,WS0,WS0,WS0,WS1,WS0,WS1,WS2,WS2,WS1,WS1,WS0,WC,WL0,WL0,WL0,WL1,WL2,], // RainCloud02
	[WL1,WL0,WL1,WL1,WL1,WL1,WL0,WC,WS0,WS0,WS0,WS1,WS2,WS0,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL0,WL0,], // Commun00
	[WL1,WL0,WL1,WL1,WL1,WL1,WL0,WC,WS0,WS0,WS0,WS1,WS2,WS0,WS1,WS2,WS1,WS0,WS0,WC,WL0,WL0,WL0,WL0,], // EventDay00
];



#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Pattern {
	Fine00 = 0,
	Fine01 = 1,
	Fine02 = 2,
	Fine03 = 3,
	Fine04 = 4,
	Fine05 = 5,
	Fine06 = 6,
	Cloud00 = 7,
	Cloud01 = 8,
	Cloud02 = 9,
	Rain00 = 10,
	Rain01 = 11,
	Rain02 = 12,
	Rain03 = 13,
	Rain04 = 14,
	Rain05 = 15,
	FineCloud00 = 16,
	FineCloud01 = 17,
	FineCloud02 = 18,
	CloudFine00 = 19,
	CloudFine01 = 20,
	CloudFine02 = 21,
	FineRain00 = 22,
	FineRain01 = 23,
	FineRain02 = 24,
	FineRain03 = 25,
	CloudRain00 = 26,
	CloudRain01 = 27,
	CloudRain02 = 28,
	RainCloud00 = 29,
	RainCloud01 = 30,
	RainCloud02 = 31,
	Commun00 = 32,
	EventDay00 = 33,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum PatternKind {
	Fine = 0, Cloud = 1, Rain = 2,
	FineCloud = 3, CloudFine = 4,
	FineRain = 5, CloudRain = 6, RainCloud = 7,
	Commun = 8, EventDay = 9
}

impl Pattern {
	pub fn from_u8(value: u8) -> Pattern {
		if value <= 33 {
			unsafe { mem::transmute(value) }
		} else {
			panic!()
		}
	}

	pub fn kind(self) -> PatternKind {
		use Pattern::*;
		use PatternKind::*;
		match self {
			Fine00 | Fine01 | Fine02 | Fine03 | Fine04 | Fine05 | Fine06 => Fine,
			Cloud00 | Cloud01 | Cloud02 => Cloud,
			Rain00 | Rain01 | Rain02 | Rain03 | Rain04 | Rain05 => Rain,
			FineCloud00 | FineCloud01 | FineCloud02 => FineCloud,
			CloudFine00 | CloudFine01 | CloudFine02 => CloudFine,
			FineRain00 | FineRain01 | FineRain02 | FineRain03 => FineRain,
			CloudRain00 | CloudRain01 | CloudRain02 => CloudRain,
			RainCloud00 | RainCloud01 | RainCloud02 => RainCloud,
			Commun00 => Commun,
			EventDay00 => EventDay
		}
	}
}

#[wasm_bindgen(js_name = getPatternKind)]
pub fn get_pattern_kind(pat: Pattern) -> PatternKind {
	pat.kind()
}
