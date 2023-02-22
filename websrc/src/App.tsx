import { createResource, Suspense } from "solid-js";
import { GrpcWebImpl, TelemetryServiceClientImpl, HeatMapRequest } from './telemetry';

const fetchJokes = async () => (await echoService.getHeatMap({ name: "test" }));

const rpc = new GrpcWebImpl('http://localhost:8000', {});
const echoService = new TelemetryServiceClientImpl(rpc);

function App() {
  const [jokes] = createResource(fetchJokes);  // Here we use createResource to associate from the fetchJokes promise to the results: the jokes variable

  return (
    <>
      <h1>Welcome</h1>
      <Suspense fallback={<p>Loading...</p>}>
      {jokes()?.message}
      </Suspense>
    </>
  );
}

export default App;
