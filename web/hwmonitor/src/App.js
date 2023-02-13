import './App.css';
import React, { useState, useEffect } from 'react';
import { GreeterClient } from './helloworld_grpc_web_pb.js';
import { HelloRequest } from './helloworld_pb';

// Create a new HardwareMonitorClient like this, correct the ADDR and Port used
// If you use something else. 
var client = new GreeterClient('http://localhost:8000');

function App() {
  const [CPU, setCPU] = useState(0);

  const getStats = () => {
    var request = new HelloRequest();
    
    request.setName("test" + Math.random());
    var deadline = new Date();
    deadline.setSeconds(deadline.getSeconds() + 3);  
    client.test(request, {deadline: deadline.getTime().toString()}, (err, res) => {
      console.log("ttt");
      if (err) {
        console.error(err);
        return;
      }
      setCPU(res.getMessage() + "312312");
    });
  }
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