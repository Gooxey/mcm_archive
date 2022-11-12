const {app, BrowserWindow} = require('electron')

function createWindow () {
    // Create the browser window
    const mainWindow = new BrowserWindow({
        width: 1440,
        height: 810,
        autoHideMenuBar: true,
        icon: __dirname + '/images/icon.ico'
    })
    console.log(__dirname);

    // load the index.html of the app
    mainWindow.loadFile('index.html')
}

// when electron finished initialization create browser window
app.whenReady().then(() => {
    createWindow()
  
    app.on('activate', function () {
        // On macOS it's common to re-create a window in the app when the
        // dock icon is clicked and there are no other windows open.
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', function () {
    if (process.platform !== 'darwin') app.quit()
})