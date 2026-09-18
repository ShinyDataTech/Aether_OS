# PowerShell Automation Script for Microkernel Simulation & Screenshot Conversion

$CargoPath = "C:\Users\wei.liu\scoop\persist\rustup\.cargo\bin\cargo.exe"
$PythonPath = "C:\Users\wei.liu\AppData\Local\Programs\Python\Python312\python.exe"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host " Building & Launching Ephemeral OS Microkernel Simulation" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# Step 1: Execute Kernel Simulation
& $CargoPath run --package kernel

# Step 2: Verify Log & PPM Output
if (Test-Path "kernel_serial.log") {
    Write-Host "`n[SUCCESS]: Serial log captured in kernel_serial.log" -ForegroundColor Green
}

if (Test-Path "canvas_output.bmp") {
    Write-Host "[SUCCESS]: Framebuffer canvas captured in canvas_output.bmp" -ForegroundColor Green
    
    # Step 3: Convert BMP to PNG image artifact using Python
    $pythonCode = @"
from PIL import Image
img = Image.open('canvas_output.bmp')
img.save('framebuffer_canvas.png')
img.save('C:/Users/wei.liu/.gemini/antigravity-ide/brain/aae11f49-23ae-4055-b027-87c709cb0986/framebuffer_canvas.png')
print('[CONVERTER]: Successfully converted canvas_output.bmp to framebuffer_canvas.png artifact!')
"@
    & $PythonPath -c $pythonCode
}
