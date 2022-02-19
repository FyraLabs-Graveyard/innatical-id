project('innatical-id', [ 'rust', 'vala', 'c' ],
        version: '1.0.0',
        meson_version: '>= 0.59.0',
        default_options: [ 'warning_level=2', ],
)

i18n = import('i18n')

# Rust
subdir('crates')

# GUI/Vala
subdir('gui')
subdir('data')