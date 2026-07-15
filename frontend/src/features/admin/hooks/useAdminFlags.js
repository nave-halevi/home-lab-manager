import { useEffect, useState } from "react";
import { getAdminFlags } from "../services/adminService";
export default function useAdminFlags() { const [items,setItems]=useState([]),[loading,setLoading]=useState(true),[error,setError]=useState(null); useEffect(()=>{ getAdminFlags().then(setItems).catch(e=>setError(e.message)).finally(()=>setLoading(false)); },[]); return {items,loading,error}; }
