mod to_do;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::ItemTypes;
fn main() {
    let to_do_item = to_do_factory("shopping", TaskStatus::PENDING);

    match to_do_item {
        ItemTypes::Done(item) => {
            println!("{}", item.super_struct.title);
            println!("{}", item.super_struct.status._stringify());
        }
        ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.title);
            println!("{}", item.super_struct.status._stringify());
        }
    }
}
