import { useEffect, useState } from "react";
import { getAdminDashboard } from "../services/adminService";
export default function useAdminOverview() { const [data,setData]=useState(null),[loading,setLoading]=useState(true),[error,setError]=useState(null); useEffect(()=>{ getAdminDashboard().then(setData).catch(e=>setError(e.message)).finally(()=>setLoading(false)); },[]); return {data,loading,error}; }
