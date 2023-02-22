import logo from './logo.svg';
import styles from './App.module.css';
import { GrpcWebImpl, TelemetryServiceClientImpl, HeatMapRequest } from './telemetry';

const rpc = new GrpcWebImpl('http://localhost:8000', {'Content-Type': 'application/grpc-web+proto'});
const echoService = new TelemetryServiceClientImpl(rpc);

async function App() {
  // const request = HeatMapRequest() {
  //   name: "test"
  // };
  const response = await echoService.getHeatMap({ name: "test" });
  
  return (
    <div class={styles.App}>
      <header class={styles.header}>
        { response }
      </header>
    </div>
  );
}

export default App;
