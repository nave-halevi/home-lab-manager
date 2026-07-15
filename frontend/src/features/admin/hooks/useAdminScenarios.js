/* eslint-disable react-hooks/set-state-in-effect */
import { useCallback, useEffect, useState } from "react";
import { createScenario, deleteScenario, getAdminScenarios, updateScenario } from "../services/adminService";
export default function useAdminScenarios() {
  const [items,setItems]=useState([]),[loading,setLoading]=useState(true),[error,setError]=useState(null),[saving,setSaving]=useState(false);
  const reload=useCallback(async()=>{ try{setError(null);setItems(await getAdminScenarios());}catch(e){setError(e.message);}finally{setLoading(false);} },[]);
  useEffect(()=>{reload();},[reload]);
  const save=async(id,payload)=>{setSaving(true);setError(null);try{if(id)await updateScenario(id,payload);else await createScenario(payload);await reload();return true;}catch(e){setError(e.message);return false;}finally{setSaving(false);}};
  const remove=async(id)=>{setSaving(true);setError(null);try{await deleteScenario(id);await reload();return true;}catch(e){setError(e.message);return false;}finally{setSaving(false);}};
  return {items,loading,error,saving,save,remove};
}
