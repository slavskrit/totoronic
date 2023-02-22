import { observable, createSignal, from, createResource, Suspense } from "solid-js";
import { GrpcWebImpl, TelemetryServiceClientImpl, HeatMapRequest } from './telemetry';
import { Subject } from 'rxjs';
const fetchJokes = async () => (await echoService.GetHeatMap({ name: "test" }));

const rpc = new GrpcWebImpl('http://localhost:8000', {});
const echoService = new TelemetryServiceClientImpl(rpc);

function ComponentWithResource() {
  const [value, setValue] = createSignal("");
  const [jokes] = createResource(fetchJokes);
  return <div>{value }{jokes()?.subscribe((v) => setValue(v.message))}</div>;
}

function App() {
  return <>
    <h1>Test</h1>
    <ComponentWithResource />
  </>
}

export default App;
