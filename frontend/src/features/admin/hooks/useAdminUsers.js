import { useEffect, useState } from "react";
import { getAdminUsers } from "../services/adminService";
export default function useAdminUsers() { const [items,setItems]=useState([]),[loading,setLoading]=useState(true),[error,setError]=useState(null); useEffect(()=>{ getAdminUsers().then(setItems).catch(e=>setError(e.message)).finally(()=>setLoading(false)); },[]); return {items,loading,error}; }
