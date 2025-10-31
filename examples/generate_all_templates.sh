#!/bin/bash
# Generate all chromatic template shaders and validate them

set -e  # Exit on error

echo "🎨 Generating All Chromatic Templates"
echo "======================================"
echo ""

# Create output directory
mkdir -p generated_shaders

# Array of operations
operations=("mix" "filter" "complement" "saturate")

# Binary path
BINARY="./target/release/tiny-agent-trainer"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found at $BINARY"
    echo "   Please build first: cargo build --release"
    exit 1
fi

echo "Using binary: $BINARY"
echo ""

# Generate each template
for op in "${operations[@]}"; do
    echo "🔨 Generating: chromatic_${op}.wgsl"

    # Generate shader
    $BINARY generate \\
        --model dummy \\
        --prompt "create chromatic ${op} operation" \\
        --output "generated_shaders/chromatic_${op}.wgsl"

    # Validate shader
    echo "🔍 Validating: chromatic_${op}.wgsl"
    if $BINARY validate "generated_shaders/chromatic_${op}.wgsl"; then
        echo "✅ Valid"
    else
        echo "❌ Validation failed"
        exit 1
    fi

    echo ""
done

echo "======================================"
echo "✅ All templates generated successfully!"
echo ""
echo "Generated files:"
ls -lh generated_shaders/
echo ""
echo "💡 Use these shaders in your chromatic_cognition_core project"
