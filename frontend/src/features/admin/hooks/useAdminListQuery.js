import { useCallback, useMemo } from "react";
import { useSearchParams } from "react-router-dom";

const DEFAULT_PAGE_SIZE = "20";

export default function useAdminListQuery(filterKeys = []) {
  const [searchParams, setSearchParams] = useSearchParams();

  const params = useMemo(() => {
    const current = {
      page: Number(searchParams.get("page") || "1"),
      page_size: Number(searchParams.get("page_size") || DEFAULT_PAGE_SIZE),
    };

    filterKeys.forEach((key) => {
      current[key] = searchParams.get(key) || "";
    });

    return current;
  }, [filterKeys, searchParams]);

  const setPage = useCallback(
    (page) => {
      setSearchParams((current) => {
        const next = new URLSearchParams(current);
        next.set("page", String(page));
        if (!next.get("page_size")) next.set("page_size", DEFAULT_PAGE_SIZE);
        return next;
      });
    },
    [setSearchParams],
  );

  const setFilter = useCallback(
    (key, value) => {
      setSearchParams((current) => {
        const next = new URLSearchParams(current);
        if (value) next.set(key, value);
        else next.delete(key);
        next.set("page", "1");
        if (!next.get("page_size")) next.set("page_size", DEFAULT_PAGE_SIZE);
        return next;
      });
    },
    [setSearchParams],
  );

  return { params, setPage, setFilter };
}
