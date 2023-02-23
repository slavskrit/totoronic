import { observable, createSignal, from, createResource, Suspense } from "solid-js";
import { GrpcWebImpl, TelemetryServiceClientImpl, HeatMapRequest } from './telemetry';
import { Subject } from 'rxjs';
const fetchJokes = async () => (await echoService.GetHeatMap({ name: "test" }));

const rpc = new GrpcWebImpl('http://localhost:8000', {});
const echoService = new TelemetryServiceClientImpl(rpc);

function ComponentWithResource() {
  const [value, setValue] = createSignal("");
  const [jokes] = createResource(fetchJokes);
  return <div style={{
    width: '100%',
    'word-break': 'break-all',
}}>{value }{jokes()?.subscribe((v) => setValue(v.message))}</div>;
}

function App() {
  return <>
    <h1>Data streamed from rust server</h1>
    <ComponentWithResource />
  </>
}

export default App;
// width: 200px;
// word-break: break-all;