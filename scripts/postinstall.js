import { execFileSync } from 'node:child_process'

execFileSync('dotnet', [
    'publish',
    '-o', './dist',
    '-c', 'Release',
    '-r', 'win-x64',
    '--nologo',
    '-p:PublishTrimmed=true',
    '-p:PublishReadyToRun=true',
    '-p:PublishSingleFile=true',
    '-p:EnableCompressionInSingleFile=true',
    '--self-contained', 'true',
    './server/Steamworks.js.csproj'
])
