use crate::ram::Ram;
use crate::virtual_memory::MemoryMappedPeripheral;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

pub struct Rom<const S: usize> {
    buffer: Vec<Vec<u8>>,
    banks: usize,
    actual_bank: usize,
}

impl<const S: usize> MemoryMappedPeripheral for Rom<S> {
    fn write(&mut self, _address: u16, _data: u8) {}

    fn read(&self, address: u16) -> u8 {
        if address >= S as u16 {
            return 0xff;
        }

        self.buffer[self.actual_bank][address as usize]
    }
}

impl<const S: usize> Rom<S> {
    pub fn new(content: Vec<u8>, banks: usize) -> Self {
        Self {
            buffer: (0..banks)
                .map(|i| content[(i * S)..((i + 1) * S)].to_vec())
                .collect(),
            banks,
            actual_bank: 0,
        }
    }

    pub fn sel_bank(&mut self, bank: usize) {
        if bank >= self.banks {
            return;
        }

        self.actual_bank = bank;
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum NewLicensee {
    None,
    NintendoRnD1,
    Capcom,
    EletronicArts,
    HudsonSoft,
    b_ai,
    kss,
    pow,
    PCMComplete,
    san_x,
    KemcoJapan,
    seta,
    Viacom,
    Nintendo,
    Bandai,
    Ocean_Acclaim,
    Konami,
    Hector,
    Taito,
    Hudson,
    Banpresto,
    UbiSoft,
    Atlus,
    Malibu,
    angel,
    Bullet_Proof,
    irem,
    Absolute,
    Acclaim,
    Activision,
    AmericanSammy,
    Konami2,
    HiTechEntertainment,
    LJN,
    Matchbox,
    Mattel,
    MiltonBradley,
    Titus,
    Virgin,
    LucasArts,
    Ocean,
    EletronicArts2,
    Infogrames,
    Interplay,
    Broderbund,
    sculptured,
    sci,
    THQ,
    Accolade,
    misawa,
    lozc,
    TokumaShotenIntermedia,
    TsukudaOriginal,
    Chunsoft,
    VideoSystem,
    Ocean_Acclaim2,
    Varie,
    Yonezawa_spal,
    Kaneko,
    PackInSoft,
    BottomUp,
    Konami_YuGiOh,
}

impl From<&[char]> for NewLicensee {
    fn from(value: &[char]) -> Self {
        match value {
            ['0', '0'] | ['\0', '\0'] => NewLicensee::None,
            ['0', '1'] => NewLicensee::NintendoRnD1,
            ['0', '8'] => NewLicensee::Capcom,
            ['1', '3'] => NewLicensee::EletronicArts,
            ['1', '8'] => NewLicensee::HudsonSoft,
            ['1', '9'] => NewLicensee::b_ai,
            ['2', '0'] => NewLicensee::kss,
            ['2', '2'] => NewLicensee::pow,
            ['2', '4'] => NewLicensee::PCMComplete,
            ['2', '5'] => NewLicensee::san_x,
            ['2', '8'] => NewLicensee::KemcoJapan,
            ['2', '9'] => NewLicensee::seta,
            ['3', '0'] => NewLicensee::Viacom,
            ['3', '1'] => NewLicensee::Nintendo,
            ['3', '2'] => NewLicensee::Bandai,
            ['3', '3'] => NewLicensee::Ocean_Acclaim,
            ['3', '4'] => NewLicensee::Konami,
            ['3', '5'] => NewLicensee::Hector,
            ['3', '7'] => NewLicensee::Taito,
            ['3', '8'] => NewLicensee::Hudson,
            ['3', '9'] => NewLicensee::Banpresto,
            ['4', '1'] => NewLicensee::UbiSoft,
            ['4', '2'] => NewLicensee::Atlus,
            ['4', '4'] => NewLicensee::Malibu,
            ['4', '6'] => NewLicensee::angel,
            ['4', '7'] => NewLicensee::Bullet_Proof,
            ['4', '9'] => NewLicensee::irem,
            ['5', '0'] => NewLicensee::Absolute,
            ['5', '1'] => NewLicensee::Acclaim,
            ['5', '2'] => NewLicensee::Activision,
            ['5', '3'] => NewLicensee::AmericanSammy,
            ['5', '4'] => NewLicensee::Konami2,
            ['5', '5'] => NewLicensee::HiTechEntertainment,
            ['5', '6'] => NewLicensee::LJN,
            ['5', '7'] => NewLicensee::Matchbox,
            ['5', '8'] => NewLicensee::Mattel,
            ['5', '9'] => NewLicensee::MiltonBradley,
            ['6', '0'] => NewLicensee::Titus,
            ['6', '1'] => NewLicensee::Virgin,
            ['6', '4'] => NewLicensee::LucasArts,
            ['6', '7'] => NewLicensee::Ocean,
            ['6', '9'] => NewLicensee::EletronicArts2,
            ['7', '0'] => NewLicensee::Infogrames,
            ['7', '1'] => NewLicensee::Interplay,
            ['7', '2'] => NewLicensee::Broderbund,
            ['7', '3'] => NewLicensee::sculptured,
            ['7', '5'] => NewLicensee::sci,
            ['7', '8'] => NewLicensee::THQ,
            ['7', '9'] => NewLicensee::Accolade,
            ['8', '0'] => NewLicensee::misawa,
            ['8', '3'] => NewLicensee::lozc,
            ['8', '6'] => NewLicensee::TokumaShotenIntermedia,
            ['8', '7'] => NewLicensee::TsukudaOriginal,
            ['9', '1'] => NewLicensee::Chunsoft,
            ['9', '2'] => NewLicensee::VideoSystem,
            ['9', '3'] => NewLicensee::Ocean_Acclaim2,
            ['9', '5'] => NewLicensee::Varie,
            ['9', '6'] => NewLicensee::Yonezawa_spal,
            ['9', '7'] => NewLicensee::Kaneko,
            ['9', '9'] => NewLicensee::PackInSoft,
            ['9', 'H'] => NewLicensee::BottomUp,
            ['A', '4'] => NewLicensee::Konami_YuGiOh,
            _ => panic!("Invalid New Licensee code: {:?}", value),
        }
    }
}

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
}

impl From<u8> for CartridgeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::Mbc1,
            0x02 => CartridgeType::Mbc1Ram,
            0x03 => CartridgeType::Mbc1RamBattery,
            0x05 => CartridgeType::Mbc2,
            0x06 => CartridgeType::Mbc2Battery,
            0x08 => CartridgeType::RomRam,
            0x09 => CartridgeType::RomRamBattery,
            0x0B => CartridgeType::Mmm01,
            0x0C => CartridgeType::Mmm01Ram,
            0x0D => CartridgeType::Mmm01RamBattery,
            0x0F => CartridgeType::Mbc3TimerBattery,
            0x10 => CartridgeType::Mbc3TimerRamBattery,
            0x11 => CartridgeType::Mbc3,
            0x12 => CartridgeType::Mbc3Ram,
            0x13 => CartridgeType::Mbc3RamBattery,
            0x19 => CartridgeType::Mbc5,
            0x1A => CartridgeType::Mbc5Ram,
            0x1B => CartridgeType::Mbc5RamBattery,
            0x1C => CartridgeType::Mbc5Rumble,
            0x1D => CartridgeType::Mbc5RumbleRam,
            0x1E => CartridgeType::Mbc5RumbleRamBattery,
            0x20 => CartridgeType::Mbc6,
            0x22 => CartridgeType::Mbc7SensorRumbleRamBattery,
            0xFC => CartridgeType::PocketCamera,
            0xFD => CartridgeType::BandaiTama5,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1RamBattery,
            _ => panic!("Invalid Cartridge type: {}", value),
        }
    }
}

#[derive(Debug)]
pub enum Destination {
    Japan = 0x00,
    Overseas = 0x01,
}

impl From<u8> for Destination {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Destination::Japan,
            0x01 => Destination::Overseas,
            _ => panic!("Invalid Destination code: {}", value),
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum OldLicensee {
    None = 0x00,
    Nintendo = 0x01,
    Capcom = 0x08,
    Hot_B = 0x09,
    Jaleco = 0x0A,
    Coconuts_Japan = 0x0B,
    Elite_Systems = 0x0C,
    EA = 0x13,
    Hudsonsoft = 0x18,
    ITC_Entertainment = 0x19,
    Yanoman = 0x1A,
    Japan_Clary = 0x1D,
    Virgin_Interactive = 0x1F,
    PCM_Complete = 0x24,
    San_X = 0x25,
    Kotobuki_Systems = 0x28,
    Seta = 0x29,
    Infogrames = 0x30,
    Nintendo2 = 0x31,
    Bandai = 0x32,
    NewLicenseeCode = 0x33,
    Konami = 0x34,
    HectorSoft = 0x35,
    Capcom2 = 0x38,
    Banpresto = 0x39,
    Entertainment_i = 0x3C,
    Gremlin = 0x3E,
    Ubisoft = 0x41,
    Atlus = 0x42,
    Malibu = 0x44,
    Angel = 0x46,
    Spectrum_Holoby = 0x47,
    Irem = 0x49,
    Virgin_Interactive2 = 0x4A,
    Malibu2 = 0x4D,
    US_Gold = 0x4F,
    Absolute = 0x50,
    Acclaim = 0x51,
    Activision = 0x52,
    American_Sammy = 0x53,
    GameTek = 0x54,
    Park_Place = 0x55,
    LJN = 0x56,
    Matchbox = 0x57,
    Milton_Bradley = 0x59,
    Mindscape = 0x5A,
    Romstar = 0x5B,
    Naxat_Soft = 0x5C,
    Tradewest = 0x5D,
    Titus = 0x60,
    Virgin_Interactive3 = 0x61,
    Ocean_Interactive = 0x67,
    EA2 = 0x69,
    Elite_Systems2 = 0x6E,
    Electro_Brain = 0x6F,
    Infogrames2 = 0x70,
    Interplay = 0x71,
    Broderbund = 0x72,
    Sculptered_Soft = 0x73,
    The_Sales_Curve = 0x75,
    t_hq = 0x78,
    Accolade = 0x79,
    Triffix_Entertainment = 0x7A,
    Microprose = 0x7C,
    Kemco = 0x7F,
    Misawa_Entertainment = 0x80,
    Lozc = 0x83,
    Tokuma_Shoten_Intermedia = 0x86,
    Bullet_Proof_Software = 0x8B,
    Vic_Tokai = 0x8C,
    Ape = 0x8E,
    I_Max = 0x8F,
    Chunsoft_Co = 0x91,
    Video_System = 0x92,
    Tsubaraya_Productions_Co = 0x93,
    Varie_Corporation = 0x95,
    Yonezawa_S_Pal = 0x96,
    Kaneko = 0x97,
    Arc = 0x99,
    Nihon_Bussan = 0x9A,
    Tecmo = 0x9B,
    Imagineer = 0x9C,
    Banpresto2 = 0x9D,
    Nova = 0x9F,
    Hori_Electric = 0xA1,
    Bandai2 = 0xA2,
    Konami2 = 0xA4,
    Kawada = 0xA6,
    Takara = 0xA7,
    Technos_Japan = 0xA9,
    Broderbund2 = 0xAA,
    Toei_Animation = 0xAC,
    Toho = 0xAD,
    Namco = 0xAF,
    acclaim = 0xB0,
    ASCII_or_Nexsoft = 0xB1,
    Bandai3 = 0xB2,
    Square_Enix = 0xB4,
    HAL_Laboratory = 0xB6,
    SNK = 0xB7,
    Pony_Canyon = 0xB9,
    Culture_Brain = 0xBA,
    Sunsoft = 0xBB,
    Sony_Imagesoft = 0xBD,
    Sammy = 0xBF,
    Taito = 0xC0,
    Kemco2 = 0xC2,
    Squaresoft = 0xC3,
    Tokuma_Shoten_Intermedia2 = 0xC4,
    Data_East = 0xC5,
    Tonkinhouse = 0xC6,
    Koei = 0xC8,
    UFL = 0xC9,
    Ultra = 0xCA,
    Vap = 0xCB,
    Use_Corporation = 0xCC,
    Meldac = 0xCD,
    Pony_Canyon_or = 0xCE,
    Angel2 = 0xCF,
    Taito2 = 0xD0,
    Sofel = 0xD1,
    Quest = 0xD2,
    Sigma_Enterprises = 0xD3,
    ASK_Kodansha_Co = 0xD4,
    Naxat_Soft2 = 0xD6,
    Copya_System = 0xD7,
    Banpresto3 = 0xD9,
    Tomy = 0xDA,
    LJN2 = 0xDB,
    NCS = 0xDD,
    Human = 0xDE,
    Altron = 0xDF,
    Jaleco2 = 0xE0,
    Towa_Chiki = 0xE1,
    Yutaka = 0xE2,
    Varie = 0xE3,
    Epcoh = 0xE5,
    Athena = 0xE7,
    Asmik_ACE_Entertainment = 0xE8,
    Natsume = 0xE9,
    King_Records = 0xEA,
    Atlus2 = 0xEB,
    Epic_Sony_Records = 0xEC,
    IGS = 0xEE,
    A_Wave = 0xF0,
    Extreme_Entertainment = 0xF3,
    LJN3 = 0xFF,
}

impl From<u8> for OldLicensee {
    fn from(value: u8) -> Self {
        match value {
            0x00 => OldLicensee::None,
            0x01 => OldLicensee::Nintendo,
            0x08 => OldLicensee::Capcom,
            0x09 => OldLicensee::Hot_B,
            0x0A => OldLicensee::Jaleco,
            0x0B => OldLicensee::Coconuts_Japan,
            0x0C => OldLicensee::Elite_Systems,
            0x13 => OldLicensee::EA,
            0x18 => OldLicensee::Hudsonsoft,
            0x19 => OldLicensee::ITC_Entertainment,
            0x1A => OldLicensee::Yanoman,
            0x1D => OldLicensee::Japan_Clary,
            0x1F => OldLicensee::Virgin_Interactive,
            0x24 => OldLicensee::PCM_Complete,
            0x25 => OldLicensee::San_X,
            0x28 => OldLicensee::Kotobuki_Systems,
            0x29 => OldLicensee::Seta,
            0x30 => OldLicensee::Infogrames,
            0x31 => OldLicensee::Nintendo2,
            0x32 => OldLicensee::Bandai,
            0x33 => OldLicensee::NewLicenseeCode,
            0x34 => OldLicensee::Konami,
            0x35 => OldLicensee::HectorSoft,
            0x38 => OldLicensee::Capcom2,
            0x39 => OldLicensee::Banpresto,
            0x3C => OldLicensee::Entertainment_i,
            0x3E => OldLicensee::Gremlin,
            0x41 => OldLicensee::Ubisoft,
            0x42 => OldLicensee::Atlus,
            0x44 => OldLicensee::Malibu,
            0x46 => OldLicensee::Angel,
            0x47 => OldLicensee::Spectrum_Holoby,
            0x49 => OldLicensee::Irem,
            0x4A => OldLicensee::Virgin_Interactive2,
            0x4D => OldLicensee::Malibu2,
            0x4F => OldLicensee::US_Gold,
            0x50 => OldLicensee::Absolute,
            0x51 => OldLicensee::Acclaim,
            0x52 => OldLicensee::Activision,
            0x53 => OldLicensee::American_Sammy,
            0x54 => OldLicensee::GameTek,
            0x55 => OldLicensee::Park_Place,
            0x56 => OldLicensee::LJN,
            0x57 => OldLicensee::Matchbox,
            0x59 => OldLicensee::Milton_Bradley,
            0x5A => OldLicensee::Mindscape,
            0x5B => OldLicensee::Romstar,
            0x5C => OldLicensee::Naxat_Soft,
            0x5D => OldLicensee::Tradewest,
            0x60 => OldLicensee::Titus,
            0x61 => OldLicensee::Virgin_Interactive3,
            0x67 => OldLicensee::Ocean_Interactive,
            0x69 => OldLicensee::EA2,
            0x6E => OldLicensee::Elite_Systems2,
            0x6F => OldLicensee::Electro_Brain,
            0x70 => OldLicensee::Infogrames2,
            0x71 => OldLicensee::Interplay,
            0x72 => OldLicensee::Broderbund,
            0x73 => OldLicensee::Sculptered_Soft,
            0x75 => OldLicensee::The_Sales_Curve,
            0x78 => OldLicensee::t_hq,
            0x79 => OldLicensee::Accolade,
            0x7A => OldLicensee::Triffix_Entertainment,
            0x7C => OldLicensee::Microprose,
            0x7F => OldLicensee::Kemco,
            0x80 => OldLicensee::Misawa_Entertainment,
            0x83 => OldLicensee::Lozc,
            0x86 => OldLicensee::Tokuma_Shoten_Intermedia,
            0x8B => OldLicensee::Bullet_Proof_Software,
            0x8C => OldLicensee::Vic_Tokai,
            0x8E => OldLicensee::Ape,
            0x8F => OldLicensee::I_Max,
            0x91 => OldLicensee::Chunsoft_Co,
            0x92 => OldLicensee::Video_System,
            0x93 => OldLicensee::Tsubaraya_Productions_Co,
            0x95 => OldLicensee::Varie_Corporation,
            0x96 => OldLicensee::Yonezawa_S_Pal,
            0x97 => OldLicensee::Kaneko,
            0x99 => OldLicensee::Arc,
            0x9A => OldLicensee::Nihon_Bussan,
            0x9B => OldLicensee::Tecmo,
            0x9C => OldLicensee::Imagineer,
            0x9D => OldLicensee::Banpresto2,
            0x9F => OldLicensee::Nova,
            0xA1 => OldLicensee::Hori_Electric,
            0xA2 => OldLicensee::Bandai2,
            0xA4 => OldLicensee::Konami2,
            0xA6 => OldLicensee::Kawada,
            0xA7 => OldLicensee::Takara,
            0xA9 => OldLicensee::Technos_Japan,
            0xAA => OldLicensee::Broderbund2,
            0xAC => OldLicensee::Toei_Animation,
            0xAD => OldLicensee::Toho,
            0xAF => OldLicensee::Namco,
            0xB0 => OldLicensee::acclaim,
            0xB1 => OldLicensee::ASCII_or_Nexsoft,
            0xB2 => OldLicensee::Bandai3,
            0xB4 => OldLicensee::Square_Enix,
            0xB6 => OldLicensee::HAL_Laboratory,
            0xB7 => OldLicensee::SNK,
            0xB9 => OldLicensee::Pony_Canyon,
            0xBA => OldLicensee::Culture_Brain,
            0xBB => OldLicensee::Sunsoft,
            0xBD => OldLicensee::Sony_Imagesoft,
            0xBF => OldLicensee::Sammy,
            0xC0 => OldLicensee::Taito,
            0xC2 => OldLicensee::Kemco2,
            0xC3 => OldLicensee::Squaresoft,
            0xC4 => OldLicensee::Tokuma_Shoten_Intermedia2,
            0xC5 => OldLicensee::Data_East,
            0xC6 => OldLicensee::Tonkinhouse,
            0xC8 => OldLicensee::Koei,
            0xC9 => OldLicensee::UFL,
            0xCA => OldLicensee::Ultra,
            0xCB => OldLicensee::Vap,
            0xCC => OldLicensee::Use_Corporation,
            0xCD => OldLicensee::Meldac,
            0xCE => OldLicensee::Pony_Canyon_or,
            0xCF => OldLicensee::Angel2,
            0xD0 => OldLicensee::Taito2,
            0xD1 => OldLicensee::Sofel,
            0xD2 => OldLicensee::Quest,
            0xD3 => OldLicensee::Sigma_Enterprises,
            0xD4 => OldLicensee::ASK_Kodansha_Co,
            0xD6 => OldLicensee::Naxat_Soft2,
            0xD7 => OldLicensee::Copya_System,
            0xD9 => OldLicensee::Banpresto3,
            0xDA => OldLicensee::Tomy,
            0xDB => OldLicensee::LJN2,
            0xDD => OldLicensee::NCS,
            0xDE => OldLicensee::Human,
            0xDF => OldLicensee::Altron,
            0xE0 => OldLicensee::Jaleco2,
            0xE1 => OldLicensee::Towa_Chiki,
            0xE2 => OldLicensee::Yutaka,
            0xE3 => OldLicensee::Varie,
            0xE5 => OldLicensee::Epcoh,
            0xE7 => OldLicensee::Athena,
            0xE8 => OldLicensee::Asmik_ACE_Entertainment,
            0xE9 => OldLicensee::Natsume,
            0xEA => OldLicensee::King_Records,
            0xEB => OldLicensee::Atlus2,
            0xEC => OldLicensee::Epic_Sony_Records,
            0xEE => OldLicensee::IGS,
            0xF0 => OldLicensee::A_Wave,
            0xF3 => OldLicensee::Extreme_Entertainment,
            0xFF => OldLicensee::LJN3,
            _ => panic!("Invalid Old licensee code: {}", value),
        }
    }
}

pub struct Cartridge {
    title: String,
    manufacture: String,
    new_licensee: NewLicensee,
    cartridge_type: CartridgeType,
    destination: Destination,
    old_licensee: OldLicensee,
    mask_rom_version: u8,
    cgb_flag: u8,
    sgb_flag: u8,
    rom_bank_count: usize,
    ram_bank_count: usize,
    header_checksum: u8,
    global_checksum: u16,
    bank0: Option<Rom<0x4000>>,
    bank1: Option<Rom<0x4000>>,
    ram: Option<Ram<0x2000>>,
}

impl Cartridge {
    fn decode_rom_bank_count(bank_count: u8) -> usize {
        match bank_count {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            0x52 => 3,
            0x53 => 3,
            0x54 => 3,
            x => panic!("Invalid ROM size: {}", x),
        }
    }

    fn decode_ram_bank_count(bank_count: u8) -> usize {
        match bank_count {
            0x00 => 0,
            0x01 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            x => panic!("Invalid RAM size: {}", x),
        }
    }

    fn decode_ascii(raw_title: &[u8]) -> String {
        raw_title.iter().fold(String::new(), |mut title, &letter| {
            if letter.is_ascii() && letter != 0x00 {
                title.push(letter as char)
            }

            title
        })
    }

    fn compute_header_checksum(content: &[u8]) -> u8 {
        content.iter().fold(0u8, |checksum, byte| {
            checksum.wrapping_sub(*byte).wrapping_sub(1)
        })
    }

    pub fn load(content: &[u8]) -> Self {
        let title = Cartridge::decode_ascii(&content[0x0134..=0x0143]);
        let manufacture = Cartridge::decode_ascii(&content[0x013F..=0x0142]);
        let cgb_flag = content[0x0143];
        let new_licensee_code = &content[0x0144..=0x0145];
        let sgb_flag = content[0x0146];
        let cartridge_type = CartridgeType::from(content[0x0147]);
        let rom_banks = Cartridge::decode_rom_bank_count(content[0x0148]);
        let ram_banks = Cartridge::decode_ram_bank_count(content[0x0149]);
        let destination_code = content[0x014A];
        let old_licensee_code = content[0x014B];
        let mask_rom_version = content[0x014C];
        let header_checksum = content[0x014D];
        let global_checksum = &content[0x014E..=0x014F];
        let global_checksum = ((global_checksum[1] as u16) << 8) | (global_checksum[0] as u16);

        let bank0 = Rom::new(content[0..0x4000].to_vec(), 1);
        let bank1 = Rom::new(content[0x4000..].to_vec(), rom_banks - 1);
        let ram = if ram_banks > 0 {
            Some(Ram::new(ram_banks))
        } else {
            None
        };

        let header_computed_checksum =
            Cartridge::compute_header_checksum(&content[0x0134..=0x014C]);
        if header_computed_checksum != header_checksum {
            panic!(
                "Header checksum validation failed: {}/{}",
                header_computed_checksum, header_checksum
            );
        }

        Self {
            title,
            manufacture,
            cgb_flag,
            sgb_flag,
            new_licensee: new_licensee_code
                .iter()
                .map(|x| *x as char)
                .collect::<Vec<_>>()
                .as_slice()
                .into(),
            cartridge_type,
            rom_bank_count: rom_banks,
            ram_bank_count: ram_banks,
            bank0: Some(bank0),
            bank1: Some(bank1),
            destination: destination_code.into(),
            mask_rom_version,
            header_checksum,
            global_checksum,
            ram,
            old_licensee: old_licensee_code.into(),
        }
    }

    pub fn take_bank0(&mut self) -> Rom<0x4000> {
        self.bank0.take().unwrap()
    }

    pub fn take_bank1(&mut self) -> Rom<0x4000> {
        self.bank1.take().unwrap()
    }

    pub fn take_ram(&mut self) -> Option<Ram<0x2000>> {
        self.ram.take()
    }
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}, made by {}\n\
            Old licensee code: {:?}\n\
            New licensee code: {:?}\n\
            Destination: {:?}\n\
            Cartridge type: {:?}\n\
            CGB: 0x{:02x}, SGB: 0x{:02x}\n\
            Mask ROM Version: {}\n\
            ROM: 16MiB x{}\n\
            RAM: 8MiB x{}\n\
            Header Checksum: 0x{:02x}\n\
            Global Checksum: 0x{:04x}",
            self.title,
            self.manufacture,
            self.old_licensee,
            self.new_licensee,
            self.destination,
            self.cartridge_type,
            self.cgb_flag,
            self.sgb_flag,
            self.mask_rom_version,
            self.rom_bank_count,
            self.ram_bank_count,
            self.header_checksum,
            self.global_checksum,
        )
    }
}
