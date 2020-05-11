const { spawn } = require('child_process');
const ls = spawn('ls', ['-lh', '/usr']);

ls.stdout.on('data', (data) => {
  console.log();
});

ls.stderr.on('data', (data) => {
  console.log();
});

ls.on('close', (code) => {
  console.log();
}); 
