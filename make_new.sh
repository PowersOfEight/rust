#!/usr/bin/env bash

# lets check for a project name
if [ -z "$1" ]; then
    echo "Usage: source make_new.sh [project_name]"
    return 1
fi

PROJECT_NAME=$1

# create the new rust project using cargo
cargo new "$PROJECT_NAME"
if [ $? -ne 0 ]; then
    echo "Error: Failed to create Rust project."
    return 1
fi

git add "$PROJECT_NAME/Cargo.toml" "$PROJECT_NAME/src/"
if [ $? -ne 0 ]; then
    echo "Error: faled to stage the files in the parent Git repository."
    return 1
fi

git commit -m "Added new Rust project: $PROJECT_NAME"
if [ $? -ne 0 ]; then
    echo "Error: Failed to commit changes."
    return 1
fi

echo "Project $PROJECT_NAME has been created and added to the parent Git repo."

