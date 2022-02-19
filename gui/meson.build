sources = [
    'main.vala',
    'window.vala',
    'application.vala',
]

deps = [
    dependency('gtk4'),
    dependency('libadwaita-1'),
]

gnome = import('gnome')
gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)

sources += gnome.compile_resources(
    'resources',
    'com.innatical.id.Settings.gresource.xml',
    c_name: 'innatical_id',
)

executable(
    meson.project_name() + '-settings',
    sources,
    vala_args: '--target-glib=2.50',
    dependencies: deps,
    install: true,
)
