use int_enum::IntEnum;

use momoden_password::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub(crate) enum Spell {
    Kintan = 0,
    Rokkaku = 1,
    Inazuma = 2,
    Hien = 3,
    Mankintan = 4,
    Fuyuu = 5,
    Dadadidi = 6,
    Houhi = 7,
}

impl Spell {
    pub(crate) fn all() -> [Self; 8] {
        std::array::from_fn(|i| Self::from_int(i as u8).unwrap())
    }
}

impl std::ops::Index<Spell> for Spells {
    type Output = bool;

    fn index(&self, spell: Spell) -> &Self::Output {
        use Spell::*;

        match spell {
            Kintan => &self.kintan,
            Rokkaku => &self.rokkaku,
            Inazuma => &self.inazuma,
            Hien => &self.hien,
            Mankintan => &self.mankintan,
            Fuyuu => &self.fuyuu,
            Dadadidi => &self.dadadidi,
            Houhi => &self.houhi,
        }
    }
}

impl std::ops::IndexMut<Spell> for Spells {
    fn index_mut(&mut self, spell: Spell) -> &mut Self::Output {
        use Spell::*;

        match spell {
            Kintan => &mut self.kintan,
            Rokkaku => &mut self.rokkaku,
            Inazuma => &mut self.inazuma,
            Hien => &mut self.hien,
            Mankintan => &mut self.mankintan,
            Fuyuu => &mut self.fuyuu,
            Dadadidi => &mut self.dadadidi,
            Houhi => &mut self.houhi,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub(crate) enum Event {
    Hanasaka = 0,
    Kintaro = 1,
    Urashima = 2,
    Netaro = 3,
    Murata = 4,
    Sarukani = 5,
    Dragon = 6,
    Hohoemi = 7,
}

impl Event {
    pub(crate) fn all() -> [Self; 8] {
        std::array::from_fn(|i| Self::from_int(i as u8).unwrap())
    }
}

impl std::ops::Index<Event> for Events {
    type Output = bool;

    fn index(&self, event: Event) -> &Self::Output {
        use Event::*;

        match event {
            Hanasaka => &self.hanasaka,
            Kintaro => &self.kintaro,
            Urashima => &self.urashima,
            Netaro => &self.netaro,
            Murata => &self.murata,
            Sarukani => &self.sarukani,
            Dragon => &self.dragon,
            Hohoemi => &self.hohoemi,
        }
    }
}

impl std::ops::IndexMut<Event> for Events {
    fn index_mut(&mut self, event: Event) -> &mut Self::Output {
        use Event::*;

        match event {
            Hanasaka => &mut self.hanasaka,
            Kintaro => &mut self.kintaro,
            Urashima => &mut self.urashima,
            Netaro => &mut self.netaro,
            Murata => &mut self.murata,
            Sarukani => &mut self.sarukani,
            Dragon => &mut self.dragon,
            Hohoemi => &mut self.hohoemi,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub(crate) enum Treasure {
    Dragon = 0,
    Fur = 1,
    Hotoke = 2,
    Hourai = 3,
    Swallow = 4,
}

impl Treasure {
    pub(crate) fn all() -> [Self; 5] {
        std::array::from_fn(|i| Self::from_int(i as u8).unwrap())
    }
}

impl std::ops::Index<Treasure> for Treasures {
    type Output = bool;

    fn index(&self, treasure: Treasure) -> &Self::Output {
        use Treasure::*;

        match treasure {
            Dragon => &self.dragon,
            Fur => &self.fur,
            Hotoke => &self.hotoke,
            Hourai => &self.hourai,
            Swallow => &self.swallow,
        }
    }
}

impl std::ops::IndexMut<Treasure> for Treasures {
    fn index_mut(&mut self, treasure: Treasure) -> &mut Self::Output {
        use Treasure::*;

        match treasure {
            Dragon => &mut self.dragon,
            Fur => &mut self.fur,
            Hotoke => &mut self.hotoke,
            Hourai => &mut self.hourai,
            Swallow => &mut self.swallow,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub(crate) enum Minion {
    Dog = 0,
    Pheasant = 1,
    Monkey = 2,
}

impl Minion {
    pub(crate) fn all() -> [Self; 3] {
        std::array::from_fn(|i| Self::from_int(i as u8).unwrap())
    }
}

impl std::ops::Index<Minion> for Minions {
    type Output = bool;

    fn index(&self, minion: Minion) -> &Self::Output {
        use Minion::*;

        match minion {
            Dog => &self.dog,
            Pheasant => &self.pheasant,
            Monkey => &self.monkey,
        }
    }
}

impl std::ops::IndexMut<Minion> for Minions {
    fn index_mut(&mut self, minion: Minion) -> &mut Self::Output {
        use Minion::*;

        match minion {
            Dog => &mut self.dog,
            Pheasant => &mut self.pheasant,
            Monkey => &mut self.monkey,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub(crate) enum Bookmark {
    Tabidachi = 0,
    Hanasaka = 1,
    Kintaro = 2,
    Urashima = 3,
    Netaro = 4,
    Kibou = 5,
    Sarukani = 6,
    Taketori = 7,
    Hohoemi = 8,
    Hien = 9,
}

impl Bookmark {
    pub(crate) fn all() -> [Self; 10] {
        std::array::from_fn(|i| Self::from_int(i as u8).unwrap())
    }
}

impl std::ops::Index<Bookmark> for Bookmarks {
    type Output = bool;

    fn index(&self, bookmark: Bookmark) -> &Self::Output {
        use Bookmark::*;

        match bookmark {
            Tabidachi => &self.tabidachi,
            Hanasaka => &self.hanasaka,
            Kintaro => &self.kintaro,
            Urashima => &self.urashima,
            Netaro => &self.netaro,
            Kibou => &self.kibou,
            Sarukani => &self.sarukani,
            Taketori => &self.taketori,
            Hohoemi => &self.hohoemi,
            Hien => &self.hien,
        }
    }
}

impl std::ops::IndexMut<Bookmark> for Bookmarks {
    fn index_mut(&mut self, bookmark: Bookmark) -> &mut Self::Output {
        use Bookmark::*;

        match bookmark {
            Tabidachi => &mut self.tabidachi,
            Hanasaka => &mut self.hanasaka,
            Kintaro => &mut self.kintaro,
            Urashima => &mut self.urashima,
            Netaro => &mut self.netaro,
            Kibou => &mut self.kibou,
            Sarukani => &mut self.sarukani,
            Taketori => &mut self.taketori,
            Hohoemi => &mut self.hohoemi,
            Hien => &mut self.hien,
        }
    }
}

pub(crate) fn spell_name(spell: Spell) -> &'static str {
    use Spell::*;

    match spell {
        Kintan => "きんたん",
        Rokkaku => "ろっかく",
        Inazuma => "いなずま",
        Hien => "ひえん",
        Mankintan => "まんきんたん",
        Fuyuu => "ふゆう",
        Dadadidi => "だだぢぢ",
        Houhi => "ほうひ",
    }
}

pub(crate) fn event_name(event: Event) -> &'static str {
    use Event::*;

    match event {
        Hanasaka => "花咲かの村で銀の鬼を倒した",
        Kintaro => "金太郎の村で金の鬼を倒した",
        Urashima => "浦島の村でパールの鬼を倒した",
        Netaro => "寝太郎を起こした",
        Murata => "寝太郎の村で村田の情報を聞いた",
        Sarukani => "やまんばを倒した",
        Dragon => "寝太郎の村で龍の首飾りを盗まれた",
        Hohoemi => "微笑みの村の通行許可を得た",
    }
}

pub(crate) fn treasure_name(treasure: Treasure) -> &'static str {
    use Treasure::*;

    match treasure {
        Dragon => "龍の首飾り",
        Fur => "金色の毛皮",
        Hotoke => "仏の御鉢",
        Hourai => "蓬莱の玉",
        Swallow => "燕の子安貝",
    }
}

pub(crate) fn minion_name(minion: Minion) -> &'static str {
    use Minion::*;

    match minion {
        Dog => "犬",
        Pheasant => "キジ",
        Monkey => "猿",
    }
}

pub(crate) fn bookmark_name(bookmark: Bookmark) -> &'static str {
    use Bookmark::*;

    match bookmark {
        Tabidachi => "旅立ちの村",
        Hanasaka => "花咲かの村",
        Kintaro => "金太郎の村",
        Urashima => "浦島の村",
        Netaro => "寝太郎の村",
        Kibou => "希望の都",
        Sarukani => "猿蟹の村",
        Taketori => "竹取の村",
        Hohoemi => "微笑みの村",
        Hien => "飛燕の城",
    }
}

pub(crate) fn respawn_name(respawn: RespawnId) -> &'static str {
    const TABLE: [&str; (RespawnId::MAX_VALUE + 1) as usize] = [
        "(海上)",
        "旅立ちの村",
        "花咲かの村",
        "金太郎の村",
        "浦島の村",
        "寝太郎の村",
        "(海上)",
        "希望の都",
        "猿蟹の村",
        "竹取の村",
        "(海上)",
        "(海上)",
        "(海上)",
        "(海上)",
        "(海上)",
        "微笑みの村",
    ];

    TABLE[usize::from(respawn)]
}

pub(crate) fn helm_index_name(helm: HelmIndex) -> &'static str {
    const TABLE: [&str; (HelmIndex::MAX_VALUE + 1) as usize] =
        ["(無装備)", "はちまき", "はちがね", "(空欄: 無視される)"];

    TABLE[usize::from(helm)]
}

pub(crate) fn weapon_index_name(weapon: WeaponIndex) -> &'static str {
    const TABLE: [&str; (WeaponIndex::MAX_VALUE + 1) as usize] = [
        "(無装備)",
        "ぼくとう",
        "かたな",
        "あすかのけん",
        "すざくのけん",
        "びゃっこのけん",
        "ひりゅうのけん",
        "あしゅらのけん",
        "ほうおうのけん",
        "オニのかなぼう",
        "ゆうきのけん",
        "(空欄: 無視される)",
        "(もものえだ: 無視される)",
        "(たけのどう: 鎧枠に装備)",
        "(あかどう: 鎧枠に装備)",
        "(むつきのどう: 鎧枠に装備)",
    ];

    TABLE[usize::from(weapon)]
}

pub(crate) fn armor_index_name(armor: ArmorIndex) -> &'static str {
    const TABLE: [&str; (ArmorIndex::MAX_VALUE + 1) as usize] = [
        "(無装備)",
        "たけのどう",
        "あかどう",
        "むつきのどう",
        "きさらぎのどう",
        "やよいのどう",
        "うづきのどう",
        "さつきのどう",
        "みなづきのどう",
        "ゆうきのどう",
        "(空欄: 無視される)",
        "(きびだんご: 無視される)",
        "(かんじき: 靴枠に装備)",
        "(ウサギのたび: 靴枠に装備)",
        "(シカのたび: 靴枠に装備)",
        "(シシのたび: 靴枠に装備)",
    ];

    TABLE[usize::from(armor)]
}

pub(crate) fn shoes_index_name(shoes: ShoesIndex) -> &'static str {
    const TABLE: [&str; (ShoesIndex::MAX_VALUE + 1) as usize] = [
        "(無装備)",
        "かんじき",
        "ウサギのたび",
        "シカのたび",
        "シシのたび",
        "(空欄: 無視される)",
        "(おにぎり: 無視される)",
        "(じんばおり: いでたち0枠に装備)",
    ];

    TABLE[usize::from(shoes)]
}

pub(crate) fn accessory0_index_name(accessory0: Accessory0Index) -> &'static str {
    const TABLE: [&str; (Accessory0Index::MAX_VALUE + 1) as usize] = [
        "(無装備)",
        "じんばおり",
        "ツルのはおり",
        "(空欄: 無視される)",
    ];

    TABLE[usize::from(accessory0)]
}

pub(crate) fn accessory1_index_name(accessory1: Accessory1Index) -> &'static str {
    const TABLE: [&str; (Accessory1Index::MAX_VALUE + 1) as usize] =
        ["(無装備)", "カイロ", "タカのつめ", "(空欄: 無視される)"];

    TABLE[usize::from(accessory1)]
}

pub(crate) fn accessory2_index_name(accessory2: Accessory2Index) -> &'static str {
    const TABLE: [&str; (Accessory2Index::MAX_VALUE + 1) as usize] = ["(無装備)", "おまもり"];

    TABLE[usize::from(accessory2)]
}

pub(crate) fn accessory3_index_name(accessory3: Accessory3Index) -> &'static str {
    const TABLE: [&str; (Accessory3Index::MAX_VALUE + 1) as usize] = ["(無装備)", "てっこう"];

    TABLE[usize::from(accessory3)]
}

pub(crate) fn item_name(id: ItemId) -> &'static str {
    const TABLE: [&str; ItemId::MAX_VALUE as usize] = [
        "おまんじゅう",
        "おにぎり",
        "きびだんご",
        "もものえだ",
        "もものみ",
        "はごろも",
        "クモのいと",
        "かんじき",
        "じんばおり",
        "ツルのはおり",
        "カイロ",
        "かくれみの",
        "おまもり",
        "はちまき",
        "はちがね",
        "てっこう",
        "たけのどう",
        "あかどう",
        "むつきのどう",
        "きさらぎのどう",
        "やよいのどう",
        "うづきのどう",
        "さつきのどう",
        "みなづきのどう",
        "ゆうきのどう",
        "ウサギのたび",
        "シカのたび",
        "シシのたび",
        "ぼくとう",
        "かたな",
        "あすかのけん",
        "すざくのけん",
        "びゃっこのけん",
        "ひりゅうのけん",
        "あしゅらのけん",
        "ほうおうのけん",
        "ぶんぶくちゃがま",
        "うちでのこづち",
        "オニのかなぼう",
        "しゃくねつのゆみや",
        "ゆうきのけん",
        "こうがしゃのタマ",
        "みずあめ",
        "ひむろのきんちゃく",
        "ひむろのけずりひ",
        "おしょくじけん",
        "タカのつめ",
        "まきもの",
        "サンゴのおふだ",
        "ユキのおにぎり",
        "せんにんのかすみ",
        "ふうりん",
        "やぐらだいこ",
        "こんろんのタマ",
        "すし",
        "うなじゅう",
        "フグりょうり",
        "しょうかいじょう",
        "つうこうてがた",
        "リュウのくびかざり",
        "キンいろのけがわ",
        "ホトケのおはち",
        "ホウライのタマ",
    ];

    TABLE[usize::from(id) - 1]
}
