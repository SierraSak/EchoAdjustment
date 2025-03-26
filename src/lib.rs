#![feature(ptr_sub_ptr)]

use engage::{
    gamedata::skill::SkillData, mapmind::MapMind,
    sequence::mapsequence::human::MapSequenceHuman,
    util::get_instance
};
use unity::prelude::OptionalMethod;

#[unity::from_offset("App", "MapMind", "get_CommandSkill")]
fn mapmind_get_commandskill(this: &MapMind, _method_info: OptionalMethod) -> &SkillData;

#[unity::from_offset("App", "MapItemMenu", "CreateBindAttack")]
fn mapitemmenu_createbindattack(sup: &MapSequenceHuman, skill: &SkillData, _method_info: OptionalMethod);


#[unity::hook("App", "MapSequenceHuman", "CreateBindAttack")]
pub fn mapsequencehuman_createbindattack_echoadjustment(this: &MapSequenceHuman, method_info: OptionalMethod) {
    let mapmind_instance = get_instance::<MapMind>();
    let skill = unsafe{mapmind_get_commandskill(mapmind_instance, method_info)};

    unsafe{mapitemmenu_createbindattack(this, skill, method_info)};
}

#[skyline::main(name = "echoadjustment")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Echo Adjustment has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Echo Adjustment has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    
    skyline::install_hooks!(
        mapsequencehuman_createbindattack_echoadjustment
    );
}
