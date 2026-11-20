use engage::app::{
    jukeboxdata::*,
    musicdata::*,
    IStructData_1Methods,
};
use engage::app::ut::prelude::IList_1Methods;
use engage::List_1Ext;

pub fn add_music_to_jukebox() {
    if JukeboxData::get_count() > 10 { return; }
    let music_list = MusicData::get_list();
    let juke_box_list = JukeboxData::get_list();
    music_list.iter().for_each(|music_data| {
       let name = music_data.get_name().to_rust_string();
        if !name.contains("FF_") && !name.contains("_ED") && !name.contains("_Evt") && !name.contains("Juke"){
            let jukebox = JukeboxData::new();
            jukebox.set_name(music_data.get_name());
            jukebox.set_event_name(music_data.get_event_name());
            juke_box_list.add(jukebox);
        }
    });
}
