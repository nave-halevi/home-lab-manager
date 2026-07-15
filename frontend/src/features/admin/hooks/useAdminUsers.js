/* eslint-disable react-hooks/exhaustive-deps */
/* eslint-disable react-hooks/set-state-in-effect */
import { useCallback, useEffect, useState } from "react";
import { getAdminUsers } from "../services/adminService";

export default function useAdminUsers(params = {}) {
  const [items, setItems] = useState([]);
  const [meta, setMeta] = useState({ page: 1, page_size: 20, total_items: 0, total_pages: 0 });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const reload = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await getAdminUsers(params);
      setItems(data.items || []);
      setMeta({
        page: data.page,
        page_size: data.page_size,
        total_items: data.total_items,
        total_pages: data.total_pages,
      });
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  }, [params.page, params.page_size, params.search, params.status]);

  useEffect(() => {
    reload();
  }, [reload]);

  return { items, meta, loading, error, reload };
}
