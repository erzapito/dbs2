import { shallowMount } from '@vue/test-utils';
import WantedItem from '@/components/wanted-item.vue';
import moxios from 'moxios';

describe('wanted-item.vue', () => {

    beforeEach( () => {
        // import and pass your custom axios instance to this method
        moxios.install();
    });
    afterEach( () => {
        // import and pass your custom axios instance to this method
        moxios.uninstall();
    });

    it('renders', (done) => {
        const wrapper = shallowMount(WantedItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "2",
                    "disc": "3",
                    "weeks": "4",
                    "done": true,
                },
            }
        });
        expect(wrapper.find("td.id").text()).toBe("1");
        expect(wrapper.find("td.artist").text()).toBe("2");
        expect(wrapper.find("td.disc").text()).toBe("3");
        expect(wrapper.find("td.weeks").text()).toBe("4");
        expect(wrapper.find("td.done").text()).toBe("true");
        done();
    });

    it ('marks wanted item', async (done) => {
        const wrapper = shallowMount(WantedItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "2",
                    "disc": "3",
                    "weeks": "4",
                    "done": true,
                },
            }
        });

        const mark = wrapper.find('.mark a');
        mark.trigger('click');
        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/wanted/1/mark');
            expect(request.config.method).toEqual('post');
            request.respondWith({
              status: 200,
              response: "",
            }).then(() => {
                expect(wrapper.emitted()["reload"]).toBeTruthy();
                done();
            });
        });
    });

    it ('downloaded wanted item', async (done) => {
        const wrapper = shallowMount(WantedItem,{
            propsData: {
                'item' : {
                    "id": 1,
                    "artist": "2",
                    "disc": "3",
                    "weeks": "4",
                    "done": true,
                },
            }
        });

        const mark = wrapper.find('.downloaded a');
        mark.trigger('click');
        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/wanted/1/downloaded');
            expect(request.config.method).toEqual('post');
            request.respondWith({
              status: 200,
              response: "",
            }).then(() => {
                expect(wrapper.emitted()["reload"]).toBeTruthy();
                done();
            });
        });
    });

});
