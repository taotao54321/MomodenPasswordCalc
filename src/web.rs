use int_enum::IntEnum as _;
use seed::{prelude::*, *};
use web_sys::HtmlSelectElement;

use momoden_password::*;

use crate::game::*;
use crate::generate::generate_passwords;
use crate::query::{Pattern, PatternChar, Query, QueryParseError};
use crate::util::{BoolExt as _, NewClampExt as _};

const PASSWORD_COUNT_MAX: usize = 100;

const CLASS_WARN: &str = "warn";
const CLASS_HAS_TOOLTIP: &str = "has-tooltip";
const CLASS_EQUIPMENT_LABEL: &str = "equipment-label";
const CLASS_EQUIPMENT_INPUT: &str = "equipment-input";

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

#[derive(Debug)]
struct Model {
    query: String,
    normalize: bool,
    savedata: Savedata,
    passwords: Vec<Password>,
    refs: Refs,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            query: Default::default(),
            normalize: true,
            savedata: Default::default(),
            passwords: Default::default(),
            refs: Default::default(),
        }
    }
}

// NOTE:
// seed はモデルが変化しても <select> 要素の表示を更新してくれない?
// (https://github.com/seed-rs/seed/issues/558)
// ブラウザによる挙動の違いもあるらしいので、<select> については自前で ElRef を持って更新する。
#[derive(Debug, Default)]
struct Refs {
    respawn_select: ElRef<HtmlSelectElement>,
    helm_select: ElRef<HtmlSelectElement>,
    weapon_select: ElRef<HtmlSelectElement>,
    armor_select: ElRef<HtmlSelectElement>,
    shoes_select: ElRef<HtmlSelectElement>,
    accessory0_select: ElRef<HtmlSelectElement>,
    accessory1_select: ElRef<HtmlSelectElement>,
    accessory2_select: ElRef<HtmlSelectElement>,
    accessory3_select: ElRef<HtmlSelectElement>,
    inventory_selects: [ElRef<HtmlSelectElement>; 8],
}

#[derive(Debug)]
enum Msg {
    ToggleNormalize,
    QueryUpdate(String),
    QuerySubmit,
    PickPassword(usize),
    SavedataUpdateXp(u16),
    SavedataUpdatePurse(u16),
    SavedataUpdateDeposit(Deposit),
    SavedataUpdateAge(u8),
    SavedataUpdateAgeTimerHi(u8),
    SavedataToggleSpell(Spell),
    SavedataToggleEvent(Event),
    SavedataToggleTreasure(Treasure),
    SavedataToggleMinion(Minion),
    SavedataToggleBookmark(Bookmark),
    SavedataUpdateRespawn(RespawnId),
    SavedataUpdateHelm(HelmIndex),
    SavedataUpdateWeapon(WeaponIndex),
    SavedataUpdateArmor(ArmorIndex),
    SavedataUpdateShoes(ShoesIndex),
    SavedataUpdateAccessory0(Accessory0Index),
    SavedataUpdateAccessory1(Accessory1Index),
    SavedataUpdateAccessory2(Accessory2Index),
    SavedataUpdateAccessory3(Accessory3Index),
    SavedataUpdateInventory(usize, Option<ItemId>),
    SavedataNormalize,
    SavedataToPassword,
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleNormalize => model.normalize.toggle(),
        Msg::QueryUpdate(query) => model.query = query,
        Msg::QuerySubmit => match Query::parse(&model.query) {
            Ok(Query::Password(password)) => {
                let Some(savedata) = load_from_password(&password, model.normalize) else {
                        return;
                    };
                update_savedata(model, savedata);
            }
            Ok(Query::Pattern(pattern)) => {
                model.passwords = generate_passwords(&pattern, PASSWORD_COUNT_MAX)
            }
            #[allow(clippy::needless_return)]
            Err(_) => return,
        },
        Msg::PickPassword(idx) => {
            let Some(password) = model.passwords.get(idx) else {
                return;
            };
            let Some(savedata) = load_from_password(password, model.normalize) else {
                return;
            };
            model.query = password.display_pretty().to_string();
            update_savedata(model, savedata);
        }
        Msg::SavedataUpdateXp(xp) => model.savedata.xp = xp,
        Msg::SavedataUpdatePurse(purse) => model.savedata.purse = purse,
        Msg::SavedataUpdateDeposit(deposit) => model.savedata.deposit = deposit,
        Msg::SavedataUpdateAge(age) => model.savedata.age = age,
        Msg::SavedataUpdateAgeTimerHi(age_timer_hi) => model.savedata.age_timer_hi = age_timer_hi,
        Msg::SavedataToggleSpell(spell) => model.savedata.spells[spell].toggle(),
        Msg::SavedataToggleEvent(event) => model.savedata.events[event].toggle(),
        Msg::SavedataToggleTreasure(treasure) => model.savedata.treasures[treasure].toggle(),
        Msg::SavedataToggleMinion(minion) => model.savedata.minions[minion].toggle(),
        Msg::SavedataToggleBookmark(bookmark) => model.savedata.bookmarks[bookmark].toggle(),
        Msg::SavedataUpdateRespawn(respawn) => {
            model.savedata.respawn = respawn;
            update_respawn_select(model);
        }
        Msg::SavedataUpdateHelm(helm) => {
            model.savedata.equipment.helm = helm;
            update_helm_select(model);
        }
        Msg::SavedataUpdateWeapon(weapon) => {
            model.savedata.equipment.weapon = weapon;
            update_weapon_select(model);
        }
        Msg::SavedataUpdateArmor(armor) => {
            model.savedata.equipment.armor = armor;
            update_armor_select(model);
        }
        Msg::SavedataUpdateShoes(shoes) => {
            model.savedata.equipment.shoes = shoes;
            update_shoes_select(model);
        }
        Msg::SavedataUpdateAccessory0(accessory0) => {
            model.savedata.equipment.accessory0 = accessory0;
            update_accessory0_select(model);
        }
        Msg::SavedataUpdateAccessory1(accessory1) => {
            model.savedata.equipment.accessory1 = accessory1;
            update_accessory1_select(model);
        }
        Msg::SavedataUpdateAccessory2(accessory2) => {
            model.savedata.equipment.accessory2 = accessory2;
            update_accessory2_select(model);
        }
        Msg::SavedataUpdateAccessory3(accessory3) => {
            model.savedata.equipment.accessory3 = accessory3;
            update_accessory3_select(model);
        }
        Msg::SavedataUpdateInventory(idx, item_id) => {
            // 空欄は詰める(原作準拠)。
            let inventory = &mut model.savedata.inventory;
            match (idx < inventory.len(), item_id) {
                (false, None) => {}
                (false, Some(item_id)) => inventory.push(item_id),
                (true, None) => {
                    inventory.remove(idx);
                }
                (true, Some(item_id)) => inventory[idx] = item_id,
            }
            update_inventory_selects(model);
        }
        Msg::SavedataNormalize => {
            model.savedata = model.savedata.normalize();
            update_helm_select(model);
            update_weapon_select(model);
            update_armor_select(model);
            update_shoes_select(model);
            update_accessory0_select(model);
            update_accessory1_select(model);
            update_accessory2_select(model);
            update_accessory3_select(model);
        }
        Msg::SavedataToPassword => {
            let password = save_to_password(&model.savedata);
            model.query = password.display_pretty().to_string();
        }
    }
}

fn update_savedata(model: &mut Model, savedata: Savedata) {
    model.savedata = savedata;
    update_respawn_select(model);
    update_helm_select(model);
    update_weapon_select(model);
    update_armor_select(model);
    update_shoes_select(model);
    update_accessory0_select(model);
    update_accessory1_select(model);
    update_accessory2_select(model);
    update_accessory3_select(model);
    update_inventory_selects(model);
}

fn update_respawn_select(model: &Model) {
    select_set_index(&model.refs.respawn_select, model.savedata.respawn);
}

fn update_helm_select(model: &Model) {
    select_set_index(&model.refs.helm_select, model.savedata.equipment.helm);
}

fn update_weapon_select(model: &Model) {
    select_set_index(&model.refs.weapon_select, model.savedata.equipment.weapon);
}

fn update_armor_select(model: &Model) {
    select_set_index(&model.refs.armor_select, model.savedata.equipment.armor);
}

fn update_shoes_select(model: &Model) {
    select_set_index(&model.refs.shoes_select, model.savedata.equipment.shoes);
}

fn update_accessory0_select(model: &Model) {
    select_set_index(
        &model.refs.accessory0_select,
        model.savedata.equipment.accessory0,
    );
}

fn update_accessory1_select(model: &Model) {
    select_set_index(
        &model.refs.accessory1_select,
        model.savedata.equipment.accessory1,
    );
}

fn update_accessory2_select(model: &Model) {
    select_set_index(
        &model.refs.accessory2_select,
        model.savedata.equipment.accessory2,
    );
}

fn update_accessory3_select(model: &Model) {
    select_set_index(
        &model.refs.accessory3_select,
        model.savedata.equipment.accessory3,
    );
}

fn update_inventory_selects(model: &Model) {
    for i in 0..8 {
        let idx = model.savedata.inventory.get(i).map_or(0, |id| id.get());
        select_set_index(&model.refs.inventory_selects[i], idx);
    }
}

fn select_set_index(er: &ElRef<HtmlSelectElement>, idx: impl Into<i32>) {
    fn f(er: &ElRef<HtmlSelectElement>, idx: i32) {
        er.get().unwrap().set_selected_index(idx);
    }

    f(er, idx.into());
}

fn load_from_password(password: &Password, normalize: bool) -> Option<Savedata> {
    let bytes = SerializedBytes::from_password(password);
    let savedata = bytes.to_savedata()?;

    Some(if normalize {
        savedata.normalize()
    } else {
        savedata
    })
}

fn save_to_password(savedata: &Savedata) -> Password {
    let bytes = SerializedBytes::from_savedata(savedata);
    bytes.to_password()
}

fn view(model: &Model) -> Node<Msg> {
    div![
        id!("app-container"),
        view_query_passwords(model),
        view_savedata(model),
    ]
}

fn view_query_passwords(model: &Model) -> Node<Msg> {
    div![
        id!("query-passwords-container"),
        view_query(model),
        h2![format!("パターン検索結果 ({PASSWORD_COUNT_MAX} 個まで)")],
        view_passwords(model),
    ]
}

fn view_query(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-query";

    div![
        id!("query-container"),
        form![
            div![
                style! {
                    St::Display => "flex",
                    St::JustifyContent => "flex-end",
                },
                view_query_normalize(model),
            ],
            div![input![
                id!(ID_INPUT),
                attrs! {
                    At::Type => "text",
                    At::Value => &model.query,
                    At::Placeholder => "パスワード ('?' は 3 個まで。空白は無視される)",
                },
                input_ev(Ev::Input, Msg::QueryUpdate)
            ]],
            view_query_ui(model),
            ev(Ev::Submit, |ev| {
                ev.prevent_default();
                Msg::QuerySubmit
            })
        ]
    ]
}

fn view_query_ui(model: &Model) -> Node<Msg> {
    match Query::parse(&model.query) {
        Ok(Query::Password(password)) => view_query_ui_password(model, &password),
        Ok(Query::Pattern(pattern)) => view_query_ui_pattern(model, &pattern),
        Err(err) => view_query_ui_error(model, &err),
    }
}

fn view_query_ui_password(_model: &Model, password: &Password) -> Node<Msg> {
    if let Some(&pc) = password.get(1) {
        if Password::is_invalid_second_char(pc) {
            return div![
                C!(CLASS_WARN),
                format!("2 文字目が '{}' のパスワードは無効", pc.to_char())
            ];
        }
    }

    if password.is_valid() {
        div![
            style! {
                St::Display => "flex",
                St::JustifyContent => "flex-end",
            },
            button![
                attrs! {
                    At::Type => "submit",
                },
                "パスワードをロード",
                ev(Ev::Click, |_| Msg::QuerySubmit)
            ]
        ]
    } else {
        div![C!(CLASS_WARN), "パスワードが無効"]
    }
}

fn view_query_ui_pattern(_model: &Model, pattern: &Pattern) -> Node<Msg> {
    if let Some(&PatternChar::Password(pc)) = pattern.get(1) {
        if Password::is_invalid_second_char(pc) {
            return div![
                C!(CLASS_WARN),
                format!("2 文字目が '{}' のパスワードは無効", pc.to_char())
            ];
        }
    }

    div![
        style! {
            St::Display => "flex",
            St::JustifyContent => "flex-end",
        },
        button![
            attrs! {
                At::Type => "submit",
            },
            "パターン検索",
            ev(Ev::Click, |_| Msg::QuerySubmit)
        ]
    ]
}

fn view_query_ui_error(_model: &Model, err: &QueryParseError) -> Node<Msg> {
    match err {
        QueryParseError::Empty => div![],
        _ => div![C!(CLASS_WARN), err.to_string()],
    }
}

fn view_query_normalize(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-normalize";
    const DESC: &str =
        "パスワードに記録された装備が不正な場合、ロード後に装備が変化する。\nチェックを外すと装備変化を行わない。";

    div![
        input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "checkbox",
                At::Checked => model.normalize.as_at_value(),
            },
            ev(Ev::Change, |_| Msg::ToggleNormalize)
        ],
        label![
            C!(CLASS_HAS_TOOLTIP),
            attrs! {
                At::For => ID_INPUT,
                At::Title => DESC,
            },
            "ロード時に正規化を行う"
        ],
    ]
}

fn view_passwords(model: &Model) -> Node<Msg> {
    const CLASS_ITEM: &str = "passwords-item";

    let passwords = model.passwords.iter().enumerate().map(|(i, password)| {
        li![
            C![CLASS_ITEM],
            password.display_pretty().to_string(),
            ev(Ev::Click, move |_| Msg::PickPassword(i))
        ]
    });

    div![id!("passwords-container"), ul![passwords]]
}

fn view_savedata(model: &Model) -> Node<Msg> {
    div![
        id!("savedata-container"),
        form![
            div![
                style! {
                    St::Display => "flex",
                    St::JustifyContent => "flex-end",
                },
                button![
                    style! {
                        St::MarginRight => "4px",
                    },
                    attrs! {
                        At::Type => "button",
                    },
                    "正規化",
                    ev(Ev::Click, |_| Msg::SavedataNormalize)
                ],
                button![
                    style! {
                        St::MarginLeft => "4px",
                    },
                    attrs! {
                        At::Type => "submit",
                    },
                    "パスワードにセーブ"
                ],
            ],
            table![
                view_savedata_xp(model),
                view_savedata_money(model),
                view_savedata_ages(model),
                view_savedata_spells(model),
                view_savedata_treasures(model),
                view_savedata_minions(model),
                view_savedata_bookmarks(model),
                view_savedata_respawn(model),
                view_savedata_events(model),
                view_savedata_equipment(model),
                view_savedata_inventory(model),
            ],
            ev(Ev::Submit, |ev| {
                ev.prevent_default();
                Msg::SavedataToPassword
            })
        ]
    ]
}

fn view_savedata_xp(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-xp";

    tr![
        th![label![
            attrs! {
                At::For => ID_INPUT,
            },
            "経験値"
        ]],
        td![input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "number",
                At::Min => u16::MIN,
                At::Max => u16::MAX,
                At::Value => model.savedata.xp,
            },
            input_ev(Ev::Change, |s| s
                .parse::<i32>()
                .ok()
                .map(u16::new_clamp)
                .map(Msg::SavedataUpdateXp))
        ]]
    ]
}

fn view_savedata_money(model: &Model) -> Node<Msg> {
    tr![
        th!["金"],
        td![div![
            id!("money-input-container"),
            view_savedata_purse(model),
            view_savedata_deposit(model),
        ]]
    ]
}

fn view_savedata_purse(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-purse";

    div![
        label![
            attrs! {
                At::For => ID_INPUT,
            },
            "所持金:"
        ],
        input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "number",
                At::Min => u16::MIN,
                At::Max => u16::MAX,
                At::Value => model.savedata.purse,
            },
            input_ev(Ev::Change, |s| s
                .parse::<i32>()
                .ok()
                .map(u16::new_clamp)
                .map(Msg::SavedataUpdatePurse))
        ],
    ]
}

fn view_savedata_deposit(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-deposit";

    div![
        label![
            attrs! {
                At::For => ID_INPUT,
            },
            "預金:"
        ],
        input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "number",
                At::Min => Deposit::MIN,
                At::Max => Deposit::MAX,
                At::Value => model.savedata.deposit,
            },
            input_ev(Ev::Change, |s| s
                .parse::<i32>()
                .ok()
                .map(Deposit::new_clamp)
                .map(Msg::SavedataUpdateDeposit))
        ],
        "000"
    ]
}

fn view_savedata_ages(model: &Model) -> Node<Msg> {
    tr![
        th!["年齢"],
        td![div![
            id!("ages-input-container"),
            view_savedata_age(model),
            view_savedata_age_timer_hi(model),
        ]]
    ]
}

fn view_savedata_age(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-age";

    div![
        input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "number",
                At::Min => u8::MIN,
                At::Max => u8::MAX,
                At::Value => model.savedata.age,
            },
            input_ev(Ev::Change, |s| s
                .parse::<i32>()
                .ok()
                .map(u8::new_clamp)
                .map(Msg::SavedataUpdateAge))
        ],
        label![
            attrs! {
                At::For => ID_INPUT,
            },
            "歳"
        ],
    ]
}

fn view_savedata_age_timer_hi(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-age-timer-hi";
    const DESC: &str = "約 25 秒で 1 増え、255 -> 0 のとき加齢する。";

    div![
        label![
            C!(CLASS_HAS_TOOLTIP),
            attrs! {
                At::For => ID_INPUT,
                At::Title => DESC,
            },
            "タイマー:"
        ],
        input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "number",
                At::Min => u8::MIN,
                At::Max => u8::MAX,
                At::Value => model.savedata.age_timer_hi,
            },
            input_ev(Ev::Change, |s| s
                .parse::<i32>()
                .ok()
                .map(u8::new_clamp)
                .map(Msg::SavedataUpdateAgeTimerHi))
        ]
    ]
}

fn view_savedata_spells(model: &Model) -> Node<Msg> {
    const ID_INPUT_BASE: &str = "input-spell";

    let inputs = Spell::all().map(|spell| {
        let id = format!("{ID_INPUT_BASE}-{}", spell.int_value());
        div![
            input![
                id!(&id),
                attrs! {
                    At::Type => "checkbox",
                    At::Checked => model.savedata.spells[spell].as_at_value(),
                },
                ev(Ev::Change, move |_| Msg::SavedataToggleSpell(spell))
            ],
            label![
                attrs! {
                    At::For => &id,
                },
                spell_name(spell)
            ],
        ]
    });

    tr![th!["術"], td![div![id!("spells-input-container"), inputs]]]
}

fn view_savedata_events(model: &Model) -> Node<Msg> {
    const ID_INPUT_BASE: &str = "input-event";

    let events = Event::all().map(|event| {
        let id = format!("{ID_INPUT_BASE}-{}", event.int_value());
        div![
            input![
                id!(&id),
                attrs! {
                    At::Type => "checkbox",
                    At::Checked => model.savedata.events[event].as_at_value(),
                },
                ev(Ev::Change, move |_| Msg::SavedataToggleEvent(event))
            ],
            label![
                attrs! {
                    At::For => &id,
                },
                event_name(event)
            ],
        ]
    });

    tr![
        th!["イベント"],
        td![
            events
            //
        ]
    ]
}

fn view_savedata_treasures(model: &Model) -> Node<Msg> {
    const ID_INPUT_BASE: &str = "input-treasure";

    let treasures = Treasure::all().map(|treasure| {
        let id = format!("{ID_INPUT_BASE}-{}", treasure.int_value());
        div![
            input![
                id!(&id),
                attrs! {
                    At::Type => "checkbox",
                    At::Checked => model.savedata.treasures[treasure].as_at_value(),
                },
                ev(Ev::Change, move |_| Msg::SavedataToggleTreasure(treasure))
            ],
            label![
                attrs! {
                    At::For => &id,
                },
                treasure_name(treasure)
            ],
        ]
    });

    tr![
        th!["宝物"],
        td![div![id!("treasures-input-container"), treasures]]
    ]
}

fn view_savedata_minions(model: &Model) -> Node<Msg> {
    const ID_INPUT_BASE: &str = "input-minion";

    let minions = Minion::all().map(|minion| {
        let id = format!("{ID_INPUT_BASE}-{}", minion.int_value());
        div![
            input![
                id!(&id),
                attrs! {
                    At::Type => "checkbox",
                    At::Checked => model.savedata.minions[minion].as_at_value(),
                },
                ev(Ev::Change, move |_| Msg::SavedataToggleMinion(minion))
            ],
            label![
                attrs! {
                    At::For => &id,
                },
                minion_name(minion)
            ],
        ]
    });

    tr![
        th!["お供"],
        td![div![id!("minions-input-container"), minions]]
    ]
}

fn view_savedata_bookmarks(model: &Model) -> Node<Msg> {
    const ID_INPUT_BASE: &str = "input-bookmark";

    let bookmarks = Bookmark::all().map(|bookmark| {
        let id = format!("{ID_INPUT_BASE}-{}", bookmark.int_value());
        div![
            input![
                id!(&id),
                attrs! {
                    At::Type => "checkbox",
                    At::Checked => model.savedata.bookmarks[bookmark].as_at_value(),
                },
                ev(Ev::Change, move |_| Msg::SavedataToggleBookmark(bookmark))
            ],
            label![
                attrs! {
                    At::For => &id,
                },
                bookmark_name(bookmark),
            ],
        ]
    });

    tr![
        th!["ブックマーク"],
        td![div![id!("bookmarks-input-container"), bookmarks]]
    ]
}

fn view_savedata_respawn(model: &Model) -> Node<Msg> {
    const ID_INPUT: &str = "input-respawn";

    let options = RespawnId::all().map(|respawn| {
        let text = format!("0x{respawn:X}: {}", respawn_name(respawn));
        let selected = respawn == model.savedata.respawn;
        option![
            attrs! {
                At::Value => respawn,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    tr![
        th![label![
            attrs! {
                At::For => ID_INPUT,
            },
            "復活地点"
        ]],
        td![select![
            el_ref(&model.refs.respawn_select),
            id!(ID_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<RespawnId>()
                .ok()
                .map(Msg::SavedataUpdateRespawn))
        ]]
    ]
}

fn view_savedata_equipment(model: &Model) -> Node<Msg> {
    tr![
        th!["装備"],
        td![div![
            id!("equipment-input-container"),
            view_savedata_helm(model),
            view_savedata_weapon(model),
            view_savedata_armor(model),
            view_savedata_shoes(model),
            view_savedata_accessory0(model),
            view_savedata_accessory1(model),
            view_savedata_accessory2(model),
            view_savedata_accessory3(model),
        ]],
    ]
}

fn view_savedata_helm(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-helm";

    let options = HelmIndex::all().map(|helm| {
        let text = format!("0x{helm:02X}: {}", helm_index_name(helm));
        let selected = helm == model.savedata.equipment.helm;
        option![
            attrs! {
                At::Value => helm,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "兜"
            ]
        ],
        div![select![
            el_ref(&model.refs.helm_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<HelmIndex>()
                .ok()
                .map(Msg::SavedataUpdateHelm))
        ]],
    ]
}

fn view_savedata_weapon(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-weapon";

    let options = WeaponIndex::all().map(|weapon| {
        let text = format!("0x{weapon:02X}: {}", weapon_index_name(weapon));
        let selected = weapon == model.savedata.equipment.weapon;
        option![
            attrs! {
                At::Value => weapon,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "武器"
            ],
        ],
        div![select![
            el_ref(&model.refs.weapon_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<WeaponIndex>()
                .ok()
                .map(Msg::SavedataUpdateWeapon))
        ]],
    ]
}

fn view_savedata_armor(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-armor";

    let options = ArmorIndex::all().map(|armor| {
        let text = format!("0x{armor:02X}: {}", armor_index_name(armor));
        let selected = armor == model.savedata.equipment.armor;
        option![
            attrs! {
                At::Value => armor,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "鎧"
            ],
        ],
        div![select![
            el_ref(&model.refs.armor_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<ArmorIndex>()
                .ok()
                .map(Msg::SavedataUpdateArmor))
        ]],
    ]
}

fn view_savedata_shoes(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-shoes";

    let options = ShoesIndex::all().map(|shoes| {
        let text = format!("0x{shoes:02X}: {}", shoes_index_name(shoes));
        let selected = shoes == model.savedata.equipment.shoes;
        option![
            attrs! {
                At::Value => shoes,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "靴"
            ],
        ],
        div![select![
            el_ref(&model.refs.shoes_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<ShoesIndex>()
                .ok()
                .map(Msg::SavedataUpdateShoes))
        ]],
    ]
}

fn view_savedata_accessory0(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-accessory0";

    let options = Accessory0Index::all().map(|accessory0| {
        let text = format!("0x{accessory0:02X}: {}", accessory0_index_name(accessory0));
        let selected = accessory0 == model.savedata.equipment.accessory0;
        option![
            attrs! {
                At::Value => accessory0,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "いでたち0"
            ],
        ],
        div![select![
            el_ref(&model.refs.accessory0_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<Accessory0Index>()
                .ok()
                .map(Msg::SavedataUpdateAccessory0))
        ]],
    ]
}

fn view_savedata_accessory1(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-accessory1";

    let options = Accessory1Index::all().map(|accessory1| {
        let text = format!("0x{accessory1:02X}: {}", accessory1_index_name(accessory1));
        let selected = accessory1 == model.savedata.equipment.accessory1;
        option![
            attrs! {
                At::Value => accessory1,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "いでたち1"
            ],
        ],
        div![select![
            el_ref(&model.refs.accessory1_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<Accessory1Index>()
                .ok()
                .map(Msg::SavedataUpdateAccessory1))
        ]],
    ]
}

fn view_savedata_accessory2(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-accessory2";

    let options = Accessory2Index::all().map(|accessory2| {
        let text = format!("0x{accessory2:02X}: {}", accessory2_index_name(accessory2));
        let selected = accessory2 == model.savedata.equipment.accessory2;
        option![
            attrs! {
                At::Value => accessory2,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "いでたち2"
            ],
        ],
        div![select![
            el_ref(&model.refs.accessory2_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<Accessory2Index>()
                .ok()
                .map(Msg::SavedataUpdateAccessory2))
        ]],
    ]
}

fn view_savedata_accessory3(model: &Model) -> Vec<Node<Msg>> {
    const ID_INPUT: &str = "input-accessory3";

    let options = Accessory3Index::all().map(|accessory3| {
        let text = format!("0x{accessory3:02X}: {}", accessory3_index_name(accessory3));
        let selected = accessory3 == model.savedata.equipment.accessory3;
        option![
            attrs! {
                At::Value => accessory3,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    nodes![
        div![
            C!(CLASS_EQUIPMENT_LABEL),
            label![
                attrs! {
                    At::For => ID_INPUT,
                },
                "いでたち3"
            ],
        ],
        div![select![
            el_ref(&model.refs.accessory3_select),
            id!(ID_INPUT),
            C!(CLASS_EQUIPMENT_INPUT),
            options,
            input_ev(Ev::Change, |s| s
                .parse::<Accessory3Index>()
                .ok()
                .map(Msg::SavedataUpdateAccessory3))
        ]],
    ]
}

fn view_savedata_inventory(model: &Model) -> Node<Msg> {
    let items = (0..8).map(|i| div![view_savedata_inventory_item(model, i)]);

    tr![th!["インベントリ"], td![items]]
}

fn view_savedata_inventory_item(model: &Model, idx: usize) -> Node<Msg> {
    let item_id = model.savedata.inventory.get(idx).copied();

    let options = (0..=ItemId::MAX_VALUE).map(|i| {
        let id = ItemId::new(i);
        let text = format!("0x{i:02X}: {}", id.map_or("(空欄)", item_name));
        let selected = i == item_id.map_or(0, ItemId::get);
        option![
            attrs! {
                At::Value => i,
                At::Selected => selected.as_at_value(),
            },
            text
        ]
    });

    div![select![
        el_ref(&model.refs.inventory_selects[idx]),
        options,
        input_ev(Ev::Change, move |s| s
            .parse::<u8>()
            .ok()
            .and_then(|x| matches!(x, 0..=ItemId::MAX_VALUE)
                .then(|| Msg::SavedataUpdateInventory(idx, ItemId::new(x)))))
    ]]
}
