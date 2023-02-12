import logo from './logo.svg';
import './App.css';
import React, { useState, useEffect } from 'react';
import { GreeterClient } from './helloworld_grpc_web_pb.js';
import { HelloRequest } from './helloworld_pb';

// Create a new HardwareMonitorClient like this, correct the ADDR and Port used
// If you use something else. 
var client = new GreeterClient('http://localhost::50051');

function App() {
  const [CPU, setCPU] = useState(0);

  const getStats = () => {
    // Create our EmptyRequest that we will use to start the stream;
    var request = new HelloRequest();
    request.name = "test";
    // Dont worry about the empty Metadata for now, thats covered in another article :)
    var stream = client.sayHello(request, {});
    console.log(stream);
    // Start listening on the data event, this is the event that is used to notify that new data arrives
    stream.on('data', function (response) {
      // Convert Response to Object
      console.log(response);
      var stats = response.toObject();
      // Set our variable values
      setCPU(stats.message);
    });
    stream.on('status', (status) => {
      console.log(status.code);
    });
    stream.on('end', (end) => {
      console.log(end);
    });
  }
  // useEffect will make this trigger on component start
  useEffect(() => {
    getStats();
  });

  return (
    <div className="App">
      <p>CPU : {CPU}</p>
    </div>
  );
}

export default App;