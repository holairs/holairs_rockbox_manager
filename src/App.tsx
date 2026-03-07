import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const path_txt_1: String = "./test1.txt";
  const path_txt_2: String = "./test2.txt";
  const path_out: String = "../out_file_test.txt";
  const name_test: String = "Ivan";
	const songs_path: String = "./Musical.m3u8";

  async function readFile(path: String) {
    let res = await invoke("comm_read_file_lines", { path });
    console.log(res);
  }

  async function validateSongsPath(path: String) {
    let res = await invoke("comm_read_file_lines", { path });
    console.log(res);
  }

  async function mergeFiles(
    path_1_test: String,
    path_2_test: String,
    path_out_0_test: String,
  ) {
    try {
      let res = await invoke("comm_merge_playlists", {
        path1: path_1_test,
        path2: path_2_test,
        outPath: path_out_0_test,
      });
      console.log("Merge Done");
      console.log(res);
    } catch (err) {
      console.error("Error on backend (RUST):", err);
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
      <button onClick={() => readFile(path_txt_2)}>Buscar</button>
      <button onClick={() => mergeFiles(path_txt_1, path_txt_2, path_out)}>
        Merge
      </button>
      <button onClick={() => salute()}>Saludar</button>
      <button onClick={() => validateSongsPath(songs_path)}>Validar Rutas de Canción</button>
    </main>
  );
}

export default App;
