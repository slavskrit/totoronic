import { createSignal } from "solid-js";
import { lazy, Suspense } from "solid-js";
import { GrpcWebImpl, TelemetryServiceClientImpl, HeatMapRequest } from './telemetry';
import { createResource } from "solid-js";

const [count, setCount] = createSignal(0);
const [message, setMessage] = createSignal(0);

const fetchJokes = async (id) => (await echoService.getHeatMap({ name: "test" }));

const rpc = new GrpcWebImpl('http://localhost:8000', {'Content-Type': 'application/grpc-web+proto'});
const echoService = new TelemetryServiceClientImpl(rpc);

function App() {
  const [jokes] = createResource(fetchJokes);  // Here we use createResource to associate from the fetchJokes promise to the results: the jokes variable

  // const request = HeatMapRequest() {
  //   name: "test"
  // };
  // const response = ;
  // const [count, setCount] = createSignal(0),
  // timer = setInterval(() => setCount(count() + 1), 1000);
  // setMessage(response.message);
  return (
    <>
      <h1>Welcome</h1>
      <Suspense fallback={<p>Loading...</p>}>
      {JSON.stringify(jokes())}
        {/* <Greeting name="Jake" /> */}
      </Suspense>
    </>
  );
}

export default App;
