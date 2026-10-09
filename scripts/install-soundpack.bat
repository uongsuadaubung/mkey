@echo off
chcp 65001 >nul
title MKey - Cài đặt gói âm thanh phím cơ
echo ======================================================
echo    MKey — Tự động tải và cài đặt gói âm thanh
echo ======================================================
echo.
echo Đang tải thư mục switches từ GitHub repo về thư mục config...
powershell -NoProfile -ExecutionPolicy Bypass -Command "$u='https://github.com/uongsuadaubung/mkey/archive/refs/heads/main.zip'; $c=Join-Path $env:USERPROFILE '.config\mkey'; $t=Join-Path $env:TEMP ('mkey-'+[guid]::NewGuid().ToString('N')); $z=\"$t.zip\"; New-Item -ItemType Directory -Force -Path $c | Out-Null; Write-Host 'Đang kết nối GitHub...' -ForegroundColor Yellow; Invoke-WebRequest -Uri $u -OutFile $z -UseBasicParsing; Write-Host 'Đang giải nén thư mục switches...' -ForegroundColor Yellow; Expand-Archive -Path $z -DestinationPath $t -Force; $s=Get-ChildItem -Path $t -Filter 'switches' -Directory -Recurse | Select-Object -First 1; if ($s) { Copy-Item -Path $s.FullName -Destination $c -Recurse -Force; Write-Host 'Cài đặt hoàn tất thành công!' -ForegroundColor Green } else { Write-Host 'Lỗi: Không tìm thấy thư mục switches' -ForegroundColor Red }; Remove-Item $z -Force -ErrorAction SilentlyContinue; Remove-Item $t -Recurse -Force -ErrorAction SilentlyContinue"
echo.
echo Đã cài đặt xong! Mở Bảng điều khiển MKey để bật âm thanh.
echo.
pause
