const net = require("net");

let server = net.createServer(function(connection) {
    connection.on("data",(data)=>{
        data = data.toString();
        process.stdout.write(data);
    });
    connection.on("end",()=>{
        console.log("");
    });
    connection.on("error",()=>{
        console.log("");
    });
})

server.listen(4569,function(){
    console.log("------------------------server running------------------------");
})