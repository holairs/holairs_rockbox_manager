import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const path_txt_1: String = "./test1.txt";
  const path_txt_2: String = "./test2.txt";
  const path_out: String = "./out_file_test.txt";
  const name_test: String = "Ivan";

  async function readFile(path: String) {
    let res = await invoke("comm_read_file_lines", { path });
    console.log(res);
  }

  async function mergeFiles(
    path_1_test: String,
    path_2_test: String,
    path_out_0_test: String,
  ) {
    console.log("Ejecutando merge con:", {
      path_1: path_1_test,
      path_2: path_2_test,
      path_out_0: path_out_0_test,
    });
    try {
      // En App.tsx
      let res = await invoke("comm_merge_playlists", {
        path1: path_1_test, // Prueba sin el guion bajo
        path2: path_2_test, // Prueba sin el guion bajo
        outPath: path_out_0_test, // camelCase
      });
      console.log(res);
    } catch (err) {
      console.error("Error desde Rust:", err);
    }
  }

  async function salute() {
    console.log("Se ha ejecutado salute");
    let res = await invoke("comm_salute", { name: name_test });
    console.log(res);
  }

  return (
    <main className="container">
      <h1>HOLA</h1>
      <button onClick={() => readFile(path_txt_1)}>Buscar</button>
      <button onClick={() => mergeFiles(path_txt_1, path_txt_2, path_out)}>
        Merge
      </button>
      <button onClick={() => salute()}>Saludar</button>
    </main>
  );
}

export default App;
