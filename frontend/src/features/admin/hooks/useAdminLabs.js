import { useEffect, useState } from "react";
import { getAdminLabs } from "../services/adminService";
export default function useAdminLabs() { const [items,setItems]=useState([]),[loading,setLoading]=useState(true),[error,setError]=useState(null); useEffect(()=>{ getAdminLabs().then(setItems).catch(e=>setError(e.message)).finally(()=>setLoading(false)); },[]); return {items,loading,error}; }
