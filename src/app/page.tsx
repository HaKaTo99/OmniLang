import { Suspense } from 'react';
import { OmniLangStudio } from "@/app/omni-lang-studio";
import { Skeleton } from '@/components/ui/skeleton';

export default function Home() {
  return (
    <Suspense fallback={<div className="flex h-screen w-screen items-center justify-center"><Skeleton className="h-[80vh] w-[90vw] rounded-lg" /></div>}>
      <OmniLangStudio />
    </Suspense>
  );
}
