mod to_do;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::get::Get;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::traits::edit::Edit;
use to_do::ItemTypes;
fn main() {
    let to_do_item = to_do_factory("shopping", TaskStatus::DONE);

    match to_do_item {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
}
