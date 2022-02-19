sources = [
    'main.vala',
    'window.vala',
    'application.vala',
    'views/welcome.vala',
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

asresources = gnome.compile_resources(
    'as-resources', join_paths(meson.project_source_root(), 'data/com.innatical.id.Settings.gresource.xml'),
    source_dir: join_paths(meson.project_source_root(), 'data'),
    c_name: 'as'
)

executable(
    meson.project_name() + '-settings',
    sources,
    asresources,
    vala_args: '--target-glib=2.50',
    dependencies: deps,
    install: true,
)
