pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
gnome = import('gnome')

resources = gnome.compile_resources('musicus',
  'musicus.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)
