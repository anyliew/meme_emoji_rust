Remove-Item -Path "rsinfo.txt", "./src/memes.rs" -Force
Get-ChildItem -Path "./src/memes" | % { $_.BaseName } >>rsinfo.txt
$inputFile = "rsinfo.txt"
$outputFile = "./src/memes.rs"

if (-not (Test-Path $inputFile)) {
    Write-Host "文件 $inputFile 不存在！"
    pause
    exit
}

Get-Content $inputFile | ForEach-Object {
    "mod $_;"
} | Out-File -FilePath $outputFile -Encoding utf8

Write-Host "处理完成！结果已保存到 $outputFile"
pause
