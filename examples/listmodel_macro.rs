#![allow(non_snake_case)]
#[macro_use]
extern crate qml;

use qml::*;

Q_LISTMODEL!{
    pub QTestModel {
        name: String,
        number: i32
    }
}

Q_LISTMODEL!{
    pub QOneMemberModel {
        name: String
    }
}

Q_LISTMODEL_ITEM!{
    pub QOneMemberModelItem<OneMemberModelItem> {
        name: String,
    }
}

Q_LISTMODEL_ITEM!{
    pub QTestModelItem<TestModelItem> {
        name: String,
        number: i32,
    }
}

fn main() {
    let mut qqae = QmlEngine::new();
    // Usage of Q_LISTMODEL macro generated wrappers
    let mut qalm = QTestModel::new();
    qalm.insert_row("John".into(), 42);
    qalm.insert_row("Oak".into(), 505);

    // Usage of Q_LISTMODEL_ITEM macro generated wrappers
    let mut qalm_item = QTestModelItem::new();
    let item1 = TestModelItem {
        name: "foo".into(),
        number: 42
    };
    let item2 = TestModelItem {
        name: "bar".into(),
        number: 23
    };
    qalm_item.insert_item(item1);
    qalm_item.insert_item(item2);

    // `&QTestModel` implements `Into<QVariant>`
    qqae.set_and_store_property("listModel", qalm.get_qvar());

    qqae.load_file("examples/listmodel.qml");
    qalm.set_data(vec![("OMG".into(), 13317), ("HACKED".into(), 228)]);
    qalm.change_line(0, "Everything's alright".into(), 123);
    qqae.exec();
    qqae.quit();
}
