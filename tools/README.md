    # Install venv 
    python -m venv .venv

    # Install PIL
    ./.venv/bin/pip install pillow

    # Make default palette
    ./ .venv/bin/python palette_microw8.py > palette_microw8.rs

    # Make encounter palette
    ./ .venv/bin/python palette_encounter.py > palette_encounter.rs
