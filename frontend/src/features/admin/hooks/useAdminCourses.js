/* eslint-disable react-hooks/set-state-in-effect */
import { useCallback, useEffect, useState } from "react";
import { createCourse, deleteCourse, getAdminCourses, updateCourse } from "../services/adminService";
export default function useAdminCourses() {
  const [items,setItems]=useState([]),[loading,setLoading]=useState(true),[error,setError]=useState(null),[saving,setSaving]=useState(false);
  const reload=useCallback(async()=>{try{setError(null);setItems(await getAdminCourses());}catch(e){setError(e.message);}finally{setLoading(false);}},[]);
  useEffect(()=>{reload();},[reload]);
  const save=async(id,payload,setMutationError)=>{setSaving(true);setError(null);setMutationError?.(null);try{if(id)await updateCourse(id,payload);else{const created=await createCourse(payload);if(payload.is_published)await updateCourse(created.id,{...payload,is_published:true});}await reload();return true;}catch(e){if(setMutationError)setMutationError(e.message);else setError(e.message);return false;}finally{setSaving(false);}};
  const remove=async(id)=>{setSaving(true);try{await deleteCourse(id);await reload();return true;}catch(e){setError(e.message);return false;}finally{setSaving(false);}};
  return {items,loading,error,saving,save,remove,reload};
}
