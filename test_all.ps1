$examplesDir = "d:\GitHub\OmniLang\examples"
$files = Get-ChildItem -Path $examplesDir -Filter *.omni

$results = @()

foreach ($file in $files) {
    Write-Host "------------------------------------" -ForegroundColor Gray
    Write-Host "Testing $($file.Name)..." -ForegroundColor Cyan
    
    $content = Get-Content -Path $file.FullName -Raw
    $isProgram = $content.Contains("module ") -or $content.Contains("fn ")
    
    if ($isProgram) {
        Write-Host "Detected as Program script. Using 'test' command..." -ForegroundColor Yellow
        $output = cargo run -- test "$($file.FullName)" 2>&1
    }
    else {
        Write-Host "Detected as Policy intent. Using 'exec' command..." -ForegroundColor Yellow
        $output = cargo run -- exec "$($file.FullName)" 2>&1
    }
    
    $status = if ($LASTEXITCODE -eq 0) { "SUCCESS" } else { "FAILED" }
    
    $results += [PSCustomObject]@{
        Name   = $file.Name
        Type   = if ($isProgram) { "Program" } else { "Policy" }
        Status = $status
    }
    
    if ($status -eq "FAILED") {
        Write-Host "Output Error:" -ForegroundColor Red
        Write-Host $output
    }
}

$results | Export-Csv -Path "test_results.csv" -NoTypeInformation
$results | Format-Table
