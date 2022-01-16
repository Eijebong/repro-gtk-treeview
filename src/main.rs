use gtk4::{prelude::*, TreePath};
use gtk4::subclass::prelude::*;

mod imp {
    use gtk4::subclass::prelude::*;

    #[derive(Default)]
    pub struct MyTreeView {}

    #[glib::object_subclass]
    impl ObjectSubclass for MyTreeView {
        const NAME: &'static str = "MyTreeView";
        type Type = super::MyTreeView;
        type ParentType = gtk4::TreeView;
    }

    impl TreeViewImpl for MyTreeView {}
    impl WidgetImpl for MyTreeView {}
    impl ObjectImpl for MyTreeView {}
}

glib::wrapper! { pub struct MyTreeView(ObjectSubclass<imp::MyTreeView>) @extends gtk4::TreeView, gtk4::Widget, @implements gtk4::Buildable; }

impl MyTreeView {
    fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

fn main() {
    let app = gtk4::Application::default();

    app.connect_activate(|app| {
        let tree = gtk4::TreeView::default();
        // Change to this and expand first
        // let tree = MyTreeView::new();

        let model = gtk4::TreeStore::new(&[String::static_type()]);
        let iter = model.insert_with_values(None, None, &[(0, &"First".to_owned().to_value())]);
        let child = model.insert_with_values(Some(&iter), None, &[(0, &"Second".to_owned().to_value())]);

        // Name
        let name_col = gtk4::TreeViewColumn::new();
        let cell_renderer = gtk4::CellRendererText::new();
        let c = cell_renderer.upcast_ref::<gtk4::CellRenderer>();

        name_col.set_cell_data_func(c, |_column, renderer, model, iter| {
            let name = model.get_value(iter, 0).get::<String>().unwrap();
            let r = renderer.downcast_ref::<gtk4::CellRendererText>().unwrap();
            r.set_text(Some(&name));
        });
        name_col.pack_start(&cell_renderer, true);

        tree.append_column(&name_col);
        tree.set_model(Some(&model));

        let main_window = gtk4::ApplicationWindow::new(app);
        main_window.set_child(Some(&tree));
        main_window.present();
    });

    app.run();
}
