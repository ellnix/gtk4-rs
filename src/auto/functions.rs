// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use Orientation;
use PageSetup;
use PrintSettings;
use SelectionData;
use StyleContext;
use TextDirection;
use TreeModel;
use TreePath;
use Widget;
use Window;
use cairo;
use gdk;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::boxed::Box as Box_;
use std::mem;
use std::ptr;


//pub fn accel_groups_activate(object: /*Ignored*/&glib::Object, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_accel_groups_activate() }
//}

//pub fn accel_groups_from_object(object: /*Ignored*/&glib::Object) -> Vec<AccelGroup> {
//    unsafe { TODO: call gtk_sys:gtk_accel_groups_from_object() }
//}

pub fn accelerator_get_default_mod_mask() -> gdk::ModifierType {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_accelerator_get_default_mod_mask())
    }
}

pub fn accelerator_get_label(accelerator_key: u32, accelerator_mods: gdk::ModifierType) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_accelerator_get_label(accelerator_key, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_get_label_with_keycode(display: Option<&gdk::Display>, accelerator_key: u32, keycode: u32, accelerator_mods: gdk::ModifierType) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_accelerator_get_label_with_keycode(display.to_glib_none().0, accelerator_key, keycode, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_name(accelerator_key: u32, accelerator_mods: gdk::ModifierType) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_accelerator_name(accelerator_key, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_name_with_keycode(display: Option<&gdk::Display>, accelerator_key: u32, keycode: u32, accelerator_mods: gdk::ModifierType) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_accelerator_name_with_keycode(display.to_glib_none().0, accelerator_key, keycode, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_parse(accelerator: &str) -> (u32, gdk::ModifierType) {
    assert_initialized_main_thread!();
    unsafe {
        let mut accelerator_key = mem::uninitialized();
        let mut accelerator_mods = mem::uninitialized();
        gtk_sys::gtk_accelerator_parse(accelerator.to_glib_none().0, &mut accelerator_key, &mut accelerator_mods);
        (accelerator_key, from_glib(accelerator_mods))
    }
}

//pub fn accelerator_parse_with_keycode(accelerator: &str, accelerator_codes: Vec<u32>) -> (u32, gdk::ModifierType) {
//    unsafe { TODO: call gtk_sys:gtk_accelerator_parse_with_keycode() }
//}

pub fn accelerator_set_default_mod_mask(default_mod_mask: gdk::ModifierType) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_accelerator_set_default_mod_mask(default_mod_mask.to_glib());
    }
}

pub fn accelerator_valid(keyval: u32, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_accelerator_valid(keyval, modifiers.to_glib()))
    }
}

//pub fn bindings_activate(object: /*Ignored*/&glib::Object, keyval: u32, modifiers: gdk::ModifierType) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_bindings_activate() }
//}

//pub fn bindings_activate_event(object: /*Ignored*/&glib::Object, event: /*Ignored*/&mut gdk::EventKey) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_bindings_activate_event() }
//}

pub fn content_formats_add_image_targets(list: &gdk::ContentFormats, writable: bool) -> Option<gdk::ContentFormats> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_content_formats_add_image_targets(list.to_glib_none().0, writable.to_glib()))
    }
}

pub fn content_formats_add_text_targets(list: &gdk::ContentFormats) -> Option<gdk::ContentFormats> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_content_formats_add_text_targets(list.to_glib_none().0))
    }
}

pub fn content_formats_add_uri_targets(list: &gdk::ContentFormats) -> Option<gdk::ContentFormats> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gtk_sys::gtk_content_formats_add_uri_targets(list.to_glib_none().0))
    }
}

//pub fn device_grab_add<P: IsA<Widget>>(widget: &P, device: /*Ignored*/&gdk::Device, block_others: bool) {
//    unsafe { TODO: call gtk_sys:gtk_device_grab_add() }
//}

//pub fn device_grab_remove<P: IsA<Widget>>(widget: &P, device: /*Ignored*/&gdk::Device) {
//    unsafe { TODO: call gtk_sys:gtk_device_grab_remove() }
//}

pub fn disable_setlocale() {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_disable_setlocale();
    }
}

//pub fn distribute_natural_allocation(extra_space: i32, n_requested_sizes: u32, sizes: /*Ignored*/&mut RequestedSize) -> i32 {
//    unsafe { TODO: call gtk_sys:gtk_distribute_natural_allocation() }
//}

pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_events_pending())
    }
}

//pub fn get_current_event() -> /*Ignored*/Option<gdk::Event> {
//    unsafe { TODO: call gtk_sys:gtk_get_current_event() }
//}

//pub fn get_current_event_device() -> /*Ignored*/Option<gdk::Device> {
//    unsafe { TODO: call gtk_sys:gtk_get_current_event_device() }
//}

pub fn get_current_event_state() -> Option<gdk::ModifierType> {
    assert_initialized_main_thread!();
    unsafe {
        let mut state = mem::uninitialized();
        let ret = from_glib(gtk_sys::gtk_get_current_event_state(&mut state));
        if ret { Some(from_glib(state)) } else { None }
    }
}

pub fn get_current_event_time() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_get_current_event_time()
    }
}

pub fn get_debug_flags() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_get_debug_flags()
    }
}

//pub fn get_default_language() -> /*Ignored*/Option<pango::Language> {
//    unsafe { TODO: call gtk_sys:gtk_get_default_language() }
//}

//pub fn get_event_target(event: /*Ignored*/&gdk::Event) -> Option<Widget> {
//    unsafe { TODO: call gtk_sys:gtk_get_event_target() }
//}

//pub fn get_event_target_with_type(event: /*Ignored*/&gdk::Event, type_: glib::types::Type) -> Option<Widget> {
//    unsafe { TODO: call gtk_sys:gtk_get_event_target_with_type() }
//}

//pub fn get_event_widget(event: /*Ignored*/&gdk::Event) -> Option<Widget> {
//    unsafe { TODO: call gtk_sys:gtk_get_event_widget() }
//}

pub fn get_locale_direction() -> TextDirection {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_get_locale_direction())
    }
}

//pub fn get_main_thread() -> /*Ignored*/Option<glib::Thread> {
//    unsafe { TODO: call gtk_sys:gtk_get_main_thread() }
//}

pub fn grab_get_current() -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(gtk_sys::gtk_grab_get_current())
    }
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    assert_initialized_main_thread!();
    unsafe {
        let mut r = mem::uninitialized();
        let mut g = mem::uninitialized();
        let mut b = mem::uninitialized();
        gtk_sys::gtk_hsv_to_rgb(h, s, v, &mut r, &mut g, &mut b);
        (r, g, b)
    }
}

pub fn im_modules_init() {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_im_modules_init();
    }
}

pub fn init_check() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_init_check())
    }
}

pub fn is_initialized() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_is_initialized())
    }
}

pub fn main() {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_main();
    }
}

//pub fn main_do_event(event: /*Ignored*/&gdk::Event) {
//    unsafe { TODO: call gtk_sys:gtk_main_do_event() }
//}

pub fn main_iteration() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_main_iteration())
    }
}

pub fn main_iteration_do(blocking: bool) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_sys::gtk_main_iteration_do(blocking.to_glib()))
    }
}

pub fn main_level() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_main_level()
    }
}

pub fn print_run_page_setup_dialog<P: IsA<Window>>(parent: Option<&P>, page_setup: Option<&PageSetup>, settings: &PrintSettings) -> Option<PageSetup> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(gtk_sys::gtk_print_run_page_setup_dialog(parent.map(|p| p.as_ref()).to_glib_none().0, page_setup.to_glib_none().0, settings.to_glib_none().0))
    }
}

pub fn print_run_page_setup_dialog_async<P: IsA<Window>, Q: FnOnce(&PageSetup) + Send + Sync + 'static>(parent: Option<&P>, page_setup: Option<&PageSetup>, settings: &PrintSettings, done_cb: Q) {
    skip_assert_initialized!();
    let done_cb_data: Box_<Q> = Box::new(done_cb);
    unsafe extern "C" fn done_cb_func<P: IsA<Window>, Q: FnOnce(&PageSetup) + Send + Sync + 'static>(page_setup: *mut gtk_sys::GtkPageSetup, data: glib_sys::gpointer) {
        let page_setup = from_glib_borrow(page_setup);
        let callback: Box_<Q> = Box_::from_raw(data as *mut _);
        (*callback)(&page_setup);
    }
    let done_cb = Some(done_cb_func::<P, Q> as _);
    let super_callback0: Box_<Q> = done_cb_data;
    unsafe {
        gtk_sys::gtk_print_run_page_setup_dialog_async(parent.map(|p| p.as_ref()).to_glib_none().0, page_setup.to_glib_none().0, settings.to_glib_none().0, done_cb, Box::into_raw(super_callback0) as *mut _);
    }
}

//pub fn propagate_event<P: IsA<Widget>>(widget: &P, event: /*Ignored*/&gdk::Event) {
//    unsafe { TODO: call gtk_sys:gtk_propagate_event() }
//}

//pub fn rc_property_parse_border(pspec: /*Ignored*/&glib::ParamSpec, gstring: /*Ignored*/&glib::String, property_value: /*Ignored*/&mut glib::Value) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_rc_property_parse_border() }
//}

//pub fn rc_property_parse_color(pspec: /*Ignored*/&glib::ParamSpec, gstring: /*Ignored*/&glib::String, property_value: /*Ignored*/&mut glib::Value) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_rc_property_parse_color() }
//}

//pub fn rc_property_parse_enum(pspec: /*Ignored*/&glib::ParamSpec, gstring: /*Ignored*/&glib::String, property_value: /*Ignored*/&mut glib::Value) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_rc_property_parse_enum() }
//}

//pub fn rc_property_parse_flags(pspec: /*Ignored*/&glib::ParamSpec, gstring: /*Ignored*/&glib::String, property_value: /*Ignored*/&mut glib::Value) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_rc_property_parse_flags() }
//}

//pub fn rc_property_parse_requisition(pspec: /*Ignored*/&glib::ParamSpec, gstring: /*Ignored*/&glib::String, property_value: /*Ignored*/&mut glib::Value) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_rc_property_parse_requisition() }
//}

pub fn render_activity<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_activity(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_arrow<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, angle: f64, x: f64, y: f64, size: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_arrow(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), angle, x, y, size);
    }
}

pub fn render_background<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_background(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

//pub fn render_background_get_clip<P: IsA<StyleContext>>(context: &P, x: f64, y: f64, width: f64, height: f64, out_clip: /*Ignored*/gdk::Rectangle) {
//    unsafe { TODO: call gtk_sys:gtk_render_background_get_clip() }
//}

pub fn render_check<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_check(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_expander<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_expander(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_focus<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_focus(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_frame<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_frame(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_handle<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_handle(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

//pub fn render_icon<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, texture: /*Ignored*/&gdk::Texture, x: f64, y: f64) {
//    unsafe { TODO: call gtk_sys:gtk_render_icon() }
//}

//pub fn render_insertion_cursor<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, layout: /*Ignored*/&pango::Layout, index: i32, direction: /*Ignored*/pango::Direction) {
//    unsafe { TODO: call gtk_sys:gtk_render_insertion_cursor() }
//}

//pub fn render_layout<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, layout: /*Ignored*/&pango::Layout) {
//    unsafe { TODO: call gtk_sys:gtk_render_layout() }
//}

pub fn render_line<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x0: f64, y0: f64, x1: f64, y1: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_line(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x0, y0, x1, y1);
    }
}

pub fn render_option<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_option(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_slider<P: IsA<StyleContext>>(context: &P, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64, orientation: Orientation) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_render_slider(context.as_ref().to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height, orientation.to_glib());
    }
}

pub fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    assert_initialized_main_thread!();
    unsafe {
        let mut h = mem::uninitialized();
        let mut s = mem::uninitialized();
        let mut v = mem::uninitialized();
        gtk_sys::gtk_rgb_to_hsv(r, g, b, &mut h, &mut s, &mut v);
        (h, s, v)
    }
}

pub fn set_debug_flags(flags: u32) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_set_debug_flags(flags);
    }
}

//pub fn show_about_dialog<P: IsA<Window>>(parent: Option<&P>, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call gtk_sys:gtk_show_about_dialog() }
//}

pub fn show_uri_on_window<P: IsA<Window>>(parent: Option<&P>, uri: &str, timestamp: u32) -> Result<(), Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = gtk_sys::gtk_show_uri_on_window(parent.map(|p| p.as_ref()).to_glib_none().0, uri.to_glib_none().0, timestamp, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

//pub fn targets_include_image(targets: /*Ignored*/&[&gdk::Atom], writable: bool) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_targets_include_image() }
//}

//pub fn targets_include_text(targets: /*Ignored*/&[&gdk::Atom]) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_targets_include_text() }
//}

//pub fn targets_include_uri(targets: /*Ignored*/&[&gdk::Atom]) -> bool {
//    unsafe { TODO: call gtk_sys:gtk_targets_include_uri() }
//}

//pub fn test_init(argvp: /*Unimplemented*/Vec<GString>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call gtk_sys:gtk_test_init() }
//}

//pub fn test_list_all_types() -> /*Unimplemented*/CArray TypeId { ns_id: 0, id: 30 } {
//    unsafe { TODO: call gtk_sys:gtk_test_list_all_types() }
//}

pub fn test_register_all_types() {
    assert_initialized_main_thread!();
    unsafe {
        gtk_sys::gtk_test_register_all_types();
    }
}

pub fn test_widget_wait_for_draw<P: IsA<Widget>>(widget: &P) {
    skip_assert_initialized!();
    unsafe {
        gtk_sys::gtk_test_widget_wait_for_draw(widget.as_ref().to_glib_none().0);
    }
}

pub fn tree_get_row_drag_data(selection_data: &SelectionData) -> Option<(Option<TreeModel>, Option<TreePath>)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut tree_model = ptr::null_mut();
        let mut path = ptr::null_mut();
        let ret = from_glib(gtk_sys::gtk_tree_get_row_drag_data(mut_override(selection_data.to_glib_none().0), &mut tree_model, &mut path));
        if ret { Some((from_glib_none(tree_model), from_glib_full(path))) } else { None }
    }
}

pub fn tree_set_row_drag_data<P: IsA<TreeModel>>(selection_data: &SelectionData, tree_model: &P, path: &mut TreePath) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(gtk_sys::gtk_tree_set_row_drag_data(mut_override(selection_data.to_glib_none().0), tree_model.as_ref().to_glib_none().0, path.to_glib_none_mut().0))
    }
}
