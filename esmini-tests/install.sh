#!/bin/bash
# Install script for esmini-tests pipeline dependencies

echo "🔧 Installing esmini-tests dependencies..."
echo ""

# Check Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 not found. Please install Python 3.7 or higher."
    exit 1
fi

echo "✅ Python: $(python3 --version)"

# Install pip if needed
if ! python3 -m pip --version &> /dev/null; then
    echo "📦 Installing pip..."
    
    # Try different methods
    if command -v dnf &> /dev/null; then
        sudo dnf install -y python3-pip
    elif command -v yum &> /dev/null; then
        sudo yum install -y python3-pip
    elif command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y python3-pip
    else
        echo "⚠️  Package manager not recognized. Please install pip manually:"
        echo "   curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py"
        echo "   python3 get-pip.py"
        exit 1
    fi
fi

echo "✅ pip: $(python3 -m pip --version)"

# Install Python dependencies
echo "📦 Installing Python packages..."
python3 -m pip install -r requirements.txt --user

if [ $? -eq 0 ]; then
    echo "✅ Python dependencies installed"
else
    echo "❌ Failed to install Python dependencies"
    exit 1
fi

# Check for esmini
echo ""
if command -v esmini &> /dev/null; then
    echo "✅ esmini found: $(which esmini)"
else
    echo "⚠️  esmini not found in PATH"
    echo ""
    echo "To complete the pipeline, install esmini:"
    echo "  1. Download from: https://github.com/esmini/esmini/releases"
    echo "  2. Extract the archive"
    echo "  3. Add bin/ to PATH:"
    echo "     export PATH=/path/to/esmini/bin:\$PATH"
    echo ""
    echo "Note: The pipeline will work without esmini, but simulation"
    echo "      and validation steps will be skipped."
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "Run the pipeline with:"
echo "  ./run_pipeline.sh"
