import { useState,useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import styled from "styled-components";
import "./App.css";
import Switch from '@material-ui/core/Switch';
import FormGroup from '@material-ui/core/FormGroup';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import FormControl from '@material-ui/core/FormControl';
function App() {
  const [connection, setConnection] = useState("");
  const [connectionFlag, setConnectionFlag] = useState(false);
  const [host, setHost] = useState("200.1.1.150");
  const [port, setPort] = useState("8501");
  const [readAddress, setReadAddress] = useState("DM1000");
  const [readData, setReadData] = useState("");
  const [writeAddress, setwriteAddress] = useState("DM1000");
  const [writeData, setwriteData] = useState("");
  const [writeValue, setwriteValue] = useState("1");
  const [checked, setChecked] = useState(false);
  async function setIpPort() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setConnection(await invoke("change_host_port", {"host":host,"port":port}));
    setConnectionFlag(true);
  }
  
  async function setPLCData(){
    console.log("SET DATA"!)
    setwriteData(await invoke("write_plc", {"address":writeAddress,"data":writeValue}));
  }

  function handleChange(){
    console.log("CHANGE TGGL  ",checked)
    setChecked(!checked);
  };
  useEffect(() => {
    console.log("USE EFFECT!!!   ",checked)
    async function getPLCData(){
        if(checked){
          console.log("GET DATA"!)
          setReadData(await invoke("read_plc", {"address":readAddress}));
        }
      }
      const interval = setInterval(() => {
        getPLCData();
      }, 100);
      return () => clearInterval(interval);
    }, [checked]);
  return (
    <>
      <Layout>
          <IpTable>
            <input
              id="greet-input"
              onChange={(e) => setHost(e.currentTarget.value)}
              placeholder="host"
              defaultValue="200.1.1.150"
            />
            <input
              id="greet-input"
              onChange={(e) => setPort(e.currentTarget.value)}
              placeholder="port"
              defaultValue="8501"
            />
            <button 
              onClick={()=>setIpPort()}
            >IPポート設定</button>
            <p>{connection}</p> 
          </IpTable>
          <ReadData>
            <input
                onChange={(e) => setReadAddress(e.currentTarget.value)}
                placeholder="DM1000"
                defaultValue="DM1000"
                disabled={!connectionFlag}
              />
          <FormControlLabel
          value="start"
          control={<Switch color="primary" />}
          label="Start"
          labelPlacement="start"
          disabled={!connectionFlag}
          onChange={handleChange}
        />                  


            {readData}
          </ReadData>
          <SendData>
            <input
                onChange={(e) => setwriteAddress(e.currentTarget.value)}
                placeholder="DM1000"
                defaultValue="DM1000"
              />
            <input
                onChange={(e) => setwriteValue(e.currentTarget.value)}
                placeholder="1"
                defaultValue="1"
              />
              
            <button
              type="submit"
              disabled={!connectionFlag}
              onClick={setPLCData}
            >送信</button>
            {writeData}
          </SendData>
      
      </Layout>
    </>
  );
}export default App;

const IpTable = styled.div`
`;
const ReadData = styled.div`
`;
const Layout = styled.div`
`;
const SendData = styled.div`
`;
const Input = styled.input`
`;